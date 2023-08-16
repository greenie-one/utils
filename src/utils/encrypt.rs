use chacha20poly1305::{XChaCha20Poly1305, AeadCore, aead::OsRng, KeyInit};

/*
For Stream cipher implementation, https://kerkour.com/rust-file-encryption
*/
pub fn generate_nonce() -> Vec<u8> {
    XChaCha20Poly1305::generate_nonce(&mut OsRng).to_vec()
}

pub fn get_cipher() -> XChaCha20Poly1305 {
    let key = std::env::var("AES_KEY").expect("missing AES_KEY");
    // Try into u8, 32 bytes
    let key: [u8; 32] = key.as_bytes().try_into().expect("invalid AES_KEY");
    return XChaCha20Poly1305::new(&key.into());
}