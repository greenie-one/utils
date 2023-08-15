use jsonwebtoken::DecodingKey;
use lazy_static::lazy_static;
use std::fs;
use tracing::info;

lazy_static! {
    pub static ref APP_ENV: String = std::env::var("APP_ENV").expect("APP_ENV should be defined");
}

pub fn load_env() {
    info!("APP_ENV: {:?}", APP_ENV.as_str());
    dotenv::from_filename(format!("./.env.{}", APP_ENV.as_str())).unwrap();
}

lazy_static! {
    pub static ref DECODE_KEY: DecodingKey = get_keys();
}

fn get_keys() -> DecodingKey {
    let public_key_pem = fs::read("./keys/doc_depot/public_key.pem").unwrap();
    let public_key = DecodingKey::from_rsa_pem(&public_key_pem).unwrap();
    public_key
}
