use crate::utils::MessageError;
use crate::ADMIN_CONFIG;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, decode_header, Algorithm::RS256, DecodingKey, Validation};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
    pub sub: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AdminConfig {
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
}

impl AdminConfig {
    pub fn new() -> AdminConfig {
        serde_json::from_str(ADMIN_CONFIG).expect("admin config is can not be serialize")
    }
}
#[derive(Clone, Deserialize, Serialize, Debug, GraphQLObject)]
pub struct User {
    pub uid: String,
}

const CLIENT_CERT_URL: &'static str =
    "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

pub async fn get_user(req: &HttpRequest) -> anyhow::Result<User> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(MessageError::new("Authorization header not found"))?
        .to_str()?
        .strip_prefix("Bearer ")
        .ok_or(MessageError::new("Authorization header is illegal"))?
        .to_string();
    let res = actix_web::client::Client::default()
        .get(CLIENT_CERT_URL)
        .send()
        .await
        .map_err(|e| MessageError::new(&format!("{:?}", e)))?
        .json::<HashMap<String, String>>()
        .await?;
    let header = decode_header(&token)?;
    if header.alg != RS256 {
        return Err(MessageError::new("alg mismatch").into());
    }
    let kid = header.kid.ok_or(MessageError::new("message"))?;
    let key_content = res
        .get(&kid)
        .ok_or(MessageError::new("key not found"))?
        // hack for https://github.com/Keats/jsonwebtoken not supporting `CERTIFICATE` as tag name
        .replace("CERTIFICATE", "PUBLIC KEY");

    let key = DecodingKey::from_rsa_pem(key_content.as_bytes())?;

    let config = AdminConfig::new();
    let mut validation = Validation::new(RS256);
    let iss = format!("https://securetoken.google.com/{}", config.project_id);
    validation.iss = Some(iss.clone());
    validation.set_audience(&[&config.project_id]);

    let claims = decode::<Claims>(&token, &key, &validation)?.claims;
    let now = chrono::Utc::now().timestamp();
    let verified = [
        // exp	Expiration time	Must be in the future. The time is measured in seconds since the UNIX epoch.
        claims.exp > now,
        // iat	Issued-at time	Must be in the past. The time is measured in seconds since the UNIX epoch.
        claims.iat < now,
        // aud	Audience	Must be your Firebase project ID, the unique identifier for your Firebase project, which can be found in the URL of that project's console.
        &claims.aud == &config.project_id,
        // iss	Issuer	Must be "https://securetoken.google.com/<projectId>", where <projectId> is the same project ID used for aud above.
        &claims.iss == &iss,
        // sub	Subject	Must be a non-empty string and must be the uid of the user or device.
        !claims.sub.is_empty(),
    ]
    .iter()
    .fold(true, |acc, &check| acc && check);
    if verified {
        Ok(User { uid: claims.sub })
    } else {
        Err(MessageError::new("user check failed").into())
    }
}
