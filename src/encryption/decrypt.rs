use std::string::FromUtf8Error;
use aes_gcm::{aead::{consts::U12, generic_array::GenericArray, Aead}, Aes256Gcm, KeyInit};

pub fn decrypt_ciphertext(nonce: &GenericArray<u8, U12>, secure_key: &[u8; 32], cipher_text: &Vec<u8>) 
-> Result<Option<String>, FromUtf8Error>
{
    let cipher_key = GenericArray::from_slice(secure_key);
    let cipher = Aes256Gcm::new(&cipher_key);

    let res = cipher.decrypt(nonce, cipher_text.as_slice());
    match res {
        Ok(v) => Ok(Some(String::from_utf8(v)?)),
        Err(_) => {
            eprintln!("An error occured whilst decryping the ciphertext!");
            Ok(None) 
        }
    }
}
