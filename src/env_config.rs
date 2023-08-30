use chacha20poly1305::XChaCha20Poly1305;
use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;
use std::{env, fs};
use tracing::info;
use crate::utils::encrypt::get_cipher;

lazy_static! {
    pub static ref APP_ENV: String = std::env::var("APP_ENV").expect("APP_ENV should be defined");
    pub static ref REMOTE_URL: String = std::env::var("REMOTE_BASE_URL").expect("REMOTE_BASE_URL should be defined");

    // Encryption
    pub static ref JWT_KEYS: JWTKeys = JWTKeys::new();
    pub static ref CIPHER: XChaCha20Poly1305 = get_cipher();

    // Blob Storage
    pub static ref STORAGE_ACCOUNT: String = std::env::var("STORAGE_ACCOUNT").expect("STORAGE_ACCOUNT should be defined");
    pub static ref STORAGE_ACCESS_KEY: String = std::env::var("STORAGE_ACCESS_KEY").expect("STORAGE_ACCESS_KEY should be defined");
    
}

pub struct JWTKeys {
    pub decode_key: DecodingKey,
    pub encode_key: EncodingKey,
}

impl JWTKeys {
    pub fn new() -> JWTKeys {
        let mut public_key_pem = env::var("JWT_PUBLIC_KEY").map(|v| v.as_bytes().to_vec());
        let mut private_key_pem = env::var("JWT_PRIVATE_KEY").map(|v| v.as_bytes().to_vec());
    
    
        if public_key_pem.is_err() {
            public_key_pem = Ok(fs::read("./keys/doc_depot/public_key.pem").unwrap());
            private_key_pem = Ok(fs::read("./keys/doc_depot/private_key.pem").unwrap());
        }
    
        let public_key = DecodingKey::from_rsa_pem(&public_key_pem.unwrap()).unwrap();
        let private_key = EncodingKey::from_rsa_pem(&private_key_pem.unwrap()).unwrap();
        JWTKeys {
            decode_key: public_key,
            encode_key: private_key,
        }
    }
}

pub fn load_env() {
    info!("APP_ENV: {:?}", APP_ENV.as_str());
    dotenv::from_filename(format!("./.env.{}", APP_ENV.as_str())).unwrap();
}
