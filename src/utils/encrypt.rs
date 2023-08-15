use aes_gcm::{Aes256Gcm, aead::{OsRng, generic_array::GenericArray}, AeadCore, KeyInit};

pub fn generate_nonce() -> Vec<u8> {
    Aes256Gcm::generate_nonce(&mut OsRng).to_vec()
}

pub fn get_cipher() -> Aes256Gcm {
    let key = std::env::var("AES_KEY").expect("missing AES_KEY");
    return Aes256Gcm::new(&GenericArray::from_slice(key.as_bytes()));
}