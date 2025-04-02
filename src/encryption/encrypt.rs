use std::string::FromUtf8Error;

use aes_gcm::{aead::{consts::U12, generic_array::GenericArray, Aead}, Aes256Gcm, KeyInit};

use super::{kdf::generate_secure_key, nonce::generate_nonce, salt::generate_salt};

fn generate_ciphertext(plain_text: &str, secure_key: &[u8; 32], nonce: &GenericArray<u8, U12>) 
-> Option<Vec<u8>>
{
    // convert to the standard type for AES-GCM
    let cipher_key = GenericArray::from_slice(secure_key);
    let cipher = Aes256Gcm::new(&cipher_key);

    let res = cipher.encrypt(nonce, plain_text.as_bytes());
    match res {
        Ok(v) => Some(v),
        Err(_) => {
            eprintln!("Error while encrypting application password!");
            None
        } 
    }
}

pub fn encrypt_password(plain_text: &str) -> Result<Option<String>, FromUtf8Error>  {
    // generate a salt
    let salt = generate_salt();

    // generate a secure key with plain text password and salt
    let secure_key = generate_secure_key(plain_text.as_bytes(), &salt);
    
    if secure_key.is_none() {
        eprintln!("Could not generate ciphertext; secure key was None");
        return Ok(None);
    }

    let secure_key = secure_key.unwrap(); 

    // generate a nonce
    let nonce = generate_nonce();

    // encrypt plain text password using secure key, salt, and nonce
    let cipher_text = generate_ciphertext(plain_text, &secure_key, &nonce);

    match cipher_text {
        Some(v) => Ok(Some(String::from_utf8(v)?)),
        None => {
            eprintln!("Error while generating ciphertext; function returned None");
            Ok(None)
        }
    }
}
