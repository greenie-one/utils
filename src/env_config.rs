use aes_gcm::Aes256Gcm;
use jsonwebtoken::DecodingKey;
use lazy_static::lazy_static;
use std::{fs, env};
use tracing::info;

use crate::utils::encrypt::get_cipher;

lazy_static! {
    pub static ref APP_ENV: String = std::env::var("APP_ENV").expect("APP_ENV should be defined");
    pub static ref DECODE_KEY: DecodingKey = get_keys();
    pub static ref CIPER: Aes256Gcm = get_cipher();
}

pub fn load_env() {
    info!("APP_ENV: {:?}", APP_ENV.as_str());
    dotenv::from_filename(format!("./.env.{}", APP_ENV.as_str())).unwrap();
}

fn get_keys() -> DecodingKey {
    let mut public_key_pem = env::var("JWT_PUBLIC_KEY").map(|v| v.as_bytes().to_vec());

    if public_key_pem.is_err() {
        public_key_pem = Ok(fs::read("./keys/doc_depot/public_key.pem").unwrap());
    }

    let public_key = DecodingKey::from_rsa_pem(&public_key_pem.unwrap()).unwrap();
    public_key
}
