use aes_gcm::{aead::{consts::U12, generic_array::GenericArray, Aead}, Aes256Gcm, KeyInit};
use crate::base64::encode::base64_encode;
use super::{encrypted_password::EncryptedPassword, kdf::generate_secure_key, nonce::generate_nonce, salt::generate_salt};

pub fn encrypt_password(master_password: &str, application_password: &str) -> Option<EncryptedPassword> {
    // generate a salt
    let salt = generate_salt();

    // generate a secure key with plain text password and salt
    let secure_key = generate_secure_key(master_password.as_bytes(), &salt);
    
    if secure_key.is_none() {
        eprintln!("Could not generate ciphertext; secure key was None");
        return None;
    }

    let secure_key = secure_key.unwrap(); 

    // generate a nonce
    let nonce = generate_nonce();

    // encrypt plain text password using secure key, salt, and nonce
    Some(EncryptedPassword::new(&salt, &nonce, generate_ciphertext(application_password, &secure_key, &nonce)))
}

fn generate_ciphertext(plain_text: &str, secure_key: &[u8; 32], nonce: &GenericArray<u8, U12>) 
-> Option<String>
{
    // convert to the standard type for AES-GCM
    let cipher_key = GenericArray::from_slice(secure_key);
    let cipher = Aes256Gcm::new(&cipher_key);

    let res = cipher.encrypt(nonce, plain_text.as_bytes());
    match res {
        Ok(v) => Some(base64_encode(v)),
        Err(_) => {
            eprintln!("Error while encrypting application password!");
            None
        } 
    }
}
