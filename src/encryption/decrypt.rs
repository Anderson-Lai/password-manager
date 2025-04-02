use std::string::FromUtf8Error;
use aes_gcm::{aead::{consts::U12, generic_array::GenericArray, Aead}, Aes256Gcm, KeyInit};

use super::kdf::generate_secure_key;

fn decrypt_ciphertext(nonce: &GenericArray<u8, U12>, secure_key: &[u8; 32], cipher_text: &Vec<u8>) 
-> Result<Option<String>, FromUtf8Error>
{
    let cipher_key = GenericArray::from_slice(secure_key);
    let cipher = Aes256Gcm::new(&cipher_key);

    let res = cipher.decrypt(nonce, cipher_text.as_ref());
    match res {
        Ok(v) => Ok(Some(String::from_utf8(v)?)),
        Err(_) => {
            eprintln!("An error occured whilst decrypting the ciphertext!");
            Ok(None) 
        }
    }
}

pub fn decrypt_password(master_password: &str, salt: &[u8; 16], nonce: &GenericArray<u8, U12>, cipher_text: &Vec<u8>)
-> Result<Option<String>, FromUtf8Error>
{
    let secure_key = generate_secure_key(master_password.as_bytes(), salt);
    if secure_key.is_none() {
        eprintln!("An error occured whilst decrypting ciphertext; generating secure key was None");
        return Ok(None);
    }

    let secure_key = secure_key.unwrap();

    return decrypt_ciphertext(nonce, &secure_key, cipher_text);
}
