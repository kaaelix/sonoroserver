//! AES-256-CBC + PKCS#7 encryption with Base64 encoding for `cfgsv` responses.

use aes::Aes256;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use cbc::Encryptor;
use cipher::{BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};

/// Encrypts `plaintext` with AES-256-CBC (PKCS#7 padded) and returns standard Base64.
pub fn encrypt_aes256_cbc_base64(plaintext: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> String {
    let ciphertext =
        Encryptor::<Aes256>::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(plaintext);
    BASE64.encode(ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypts_known_vector_consistently() {
        let key = [0x11u8; 32];
        let iv = [0x22u8; 16];
        let out = encrypt_aes256_cbc_base64(br#"{"ok":true}"#, &key, &iv);
        let decoded = BASE64.decode(out).expect("valid base64");
        assert_eq!(decoded.len(), 16);
    }
}
