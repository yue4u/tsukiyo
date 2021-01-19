use crate::utils::MessageError;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug, Deserialize)]
struct AdminConfig {
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
}

#[derive(Debug, Clone)]
pub struct CertsResolver {
    pub certs: HashMap<String, Vec<u8>>,
    expire: i64,
}

const CLIENT_CERT_URL: &'static str =
    "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

pub static CERTS_RESOLVER: Lazy<Arc<Mutex<CertsResolver>>> =
    Lazy::new(|| Arc::new(Mutex::new(CertsResolver::new())));

impl CertsResolver {
    pub fn new() -> Self {
        Self {
            expire: 0,
            certs: HashMap::new(),
        }
    }
    pub async fn get(&mut self, id: &str, now: i64) -> anyhow::Result<&Vec<u8>> {
        if self.expire < now {
            self.certs.clear();
            self.fetch().await?;
        }
        self.certs
            .get(id)
            .ok_or(MessageError::new("no cert is found").into())
    }

    async fn fetch(&mut self) -> anyhow::Result<()> {
        let mut res = actix_web::client::Client::default()
            .get(CLIENT_CERT_URL)
            .send()
            .await
            .map_err(|e| MessageError::new(&format!("{:?}", e)))?;
        // should be "public, max-age=20349, must-revalidate, no-transform",
        self.expire = chrono::Utc::now().timestamp()
            + res
                .headers()
                .get("Cache-Control")
                .ok_or(MessageError::new("Cache-Control is not provided"))?
                .to_str()?
                .split(',')
                .fold(None, |acc, curr| {
                    if acc.is_some() {
                        acc
                    } else {
                        curr.trim().strip_prefix("max-age=")
                    }
                })
                .ok_or(MessageError::new("max age is not found"))?
                .parse::<i64>()?;
        for (kid, x509_pem) in res.json::<HashMap<String, String>>().await?.into_iter() {
            // see https://github.com/Keats/jsonwebtoken/issues/127#issuecomment-753403072
            let pem_bytes = openssl::x509::X509::from_pem(x509_pem.as_bytes())?
                .public_key()?
                .rsa()?
                .public_key_to_pem()?;
            self.certs.insert(kid, pem_bytes);
        }
        Ok(())
    }
}
