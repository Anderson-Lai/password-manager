use std::collections::HashMap;

use aes_gcm::aead::generic_array::GenericArray;
use base64::{engine::general_purpose, Engine};
use crate::{encryption::{decrypt::decrypt_password, encrypt::{encrypt_password, EncryptedPassword}}, storage::save::save_passwords_to_disk};

const MASTER_PASSWORD: &str = "happy birthday";
const APPLICATION_PASSWORD: &str = "hello123!";

pub fn test_encryption_decryption() {

    let data = encrypt_password(MASTER_PASSWORD, APPLICATION_PASSWORD); 
    let data = data.unwrap();
    println!("{}", data.cipher_text.as_ref().unwrap());

    let bytes = general_purpose::STANDARD.decode(data.cipher_text.as_ref().unwrap());

    let nonce = general_purpose::STANDARD.decode(&data.nonce);
    let nonce = GenericArray::from_slice(nonce.as_ref().unwrap());

    let decrypted = decrypt_password(MASTER_PASSWORD, &data.salt, nonce, bytes.unwrap().as_ref());
    let decrypted = decrypted.unwrap().unwrap();
    println!("{}", decrypted);

    assert!(APPLICATION_PASSWORD == decrypted);
}

pub fn test_password_saving(passwords: &mut HashMap<String, EncryptedPassword>) {
    let data = encrypt_password(MASTER_PASSWORD, APPLICATION_PASSWORD);
    let data = data.unwrap();
    
    passwords.insert(String::from("testing application"), data);
    save_passwords_to_disk(passwords).unwrap();

    println!("\n{:?}", passwords);
}
