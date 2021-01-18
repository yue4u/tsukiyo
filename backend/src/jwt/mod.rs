use crate::ADMIN_CONFIG;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{decode_header, errors, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
    pub uid: Option<String>,
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

pub fn verify_id_token(
    token: &str,
) -> Result<
    // TokenData<Claims>
    String,
    errors::Error,
> {
    let firebase_config = AdminConfig::new();
    verify(token, &firebase_config)
}

pub fn verify(
    token: &str,
    _admin_config: &AdminConfig,
) -> Result<
    String,
    // TokenData<Claims
    errors::Error,
> {
    let kid = decode_header(token)
        .map(|header| header.kid)?
        .expect("failed to decode");
    println!("{:?}", kid);
    Ok(kid)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub uid: String,
}

impl FromRequest for User {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .to_str()
            .expect("not able to no auth token found")
            .strip_prefix("Bearer ")
            .expect("Authorization header is illegal");

        let uid = verify_id_token(token).expect("failed to verify id token");
        ok(User { uid })
    }
}
