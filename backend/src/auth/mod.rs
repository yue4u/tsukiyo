use crate::utils::{MessageError, OrMessageError};
use actix_web::HttpRequest;
use jsonwebtoken::{decode, decode_header, Algorithm::RS256, DecodingKey, Validation};
use juniper::GraphQLObject;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

mod certs;

#[derive(Clone, Debug, Deserialize)]
struct AdminConfig {
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
}

#[derive(Clone, Debug)]
struct AuthConstraints {
    admin_config: AdminConfig,
    validation: Validation,
}

static AUTH_CONSTRAINTS: Lazy<AuthConstraints> = Lazy::new(|| {
    let admin_config = serde_json::from_str::<AdminConfig>(&std::env::var("ADMIN_CONFIG").unwrap())
        .expect("admin config is can not be serialize");

    let mut validation = Validation::new(RS256);
    let iss = format!("https://securetoken.google.com/{}", admin_config.project_id);
    validation.iss = Some(iss.clone());
    validation.set_audience(&[&admin_config.project_id]);

    AuthConstraints {
        admin_config,
        validation,
    }
});

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    iat: i64,
    exp: i64,
    iss: String,
    sub: String,
    auth_time: i64,
}

#[derive(Clone, Deserialize, Serialize, Debug, GraphQLObject)]
pub struct User {
    pub uid: String,
}

/// following steps in https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library
pub async fn get_user(req: &HttpRequest) -> anyhow::Result<User> {
    let token = req
        .headers()
        .get("Authorization")
        .or_error("Authorization header not found")?
        .to_str()?
        .strip_prefix("Bearer ")
        .or_error("Authorization header is illegal")?
        .to_string();
    let header = decode_header(&token)?;
    if header.alg != RS256 {
        return Err(MessageError::new("alg mismatch").into());
    }
    let kid = header.kid.or_error("message")?;
    let now = chrono::Utc::now().timestamp();

    let mut resolver = certs::CERTS_RESOLVER
        .lock()
        .ok()
        .or_error("failed obtain mutex")?;
    let pem = resolver.get(&kid, now).await?;
    let key = DecodingKey::from_rsa_pem(pem.as_slice())?;

    let claims = decode::<Claims>(&token, &key, &AUTH_CONSTRAINTS.validation)?.claims;
    // this could be done in `Validation` object in the jsonwebtoken lib
    // but it need some data structure like Option<HashSet<String>>
    // which seems it's easier just check it by ourselves
    //
    // iat	Issued-at time	Must be in the past. The time is measured in seconds since the UNIX epoch.
    let verified = claims.iat < now
        // exp	Expiration time	Must be in the future. The time is measured in seconds since the UNIX epoch.
        // already checked
        // && claims.exp > now
        // aud	Audience	Must be your Firebase project ID, the unique identifier for your Firebase project, which can be found in the URL of that project's console.
        // already checked
        // && &claims.aud == &AUTH_CONSTRAINTS.admin_config.project_id
        // iss	Issuer	Must be "https://securetoken.google.com/<projectId>", where <projectId> is the same project ID used for aud above.
        // already checked
        // && &claims.iss == &iss
        // sub	Subject	Must be a non-empty string and must be the uid of the user or device.
        && !claims.sub.is_empty()
        // auth_time Authentication time Must be in the past. The time when the user authenticated. 
        && claims.auth_time < now;
    if verified {
        Ok(User { uid: claims.sub })
    } else {
        Err(MessageError::new("user check failed").into())
    }
}
