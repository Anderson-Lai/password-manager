use aes_gcm::{aead::{consts::U12, generic_array::GenericArray, Aead}, Aes256Gcm, KeyInit};

pub fn encrypt_application_password(plain_text: &String, secure_key: &[u8; 32], nonce: &GenericArray<u8, U12>) 
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
