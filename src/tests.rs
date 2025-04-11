use std::collections::HashMap;
use aes_gcm::aead::generic_array::GenericArray;
use crate::{base64::decode::base64_decode, encryption::{decrypt::decrypt_password, encrypt::{encrypt_password, EncryptedPassword}}, password::{delete::delete_password, read::read_password, update::create_update_password}, storage::save::save_passwords_to_disk};

const MASTER_PASSWORD: &str = "happy birthday";
const APPLICATION_PASSWORD: &str = "hello123!";

pub fn test_encryption_decryption() {

    let data = encrypt_password(MASTER_PASSWORD, APPLICATION_PASSWORD); 
    let data = data.unwrap();
    println!("{}", data.cipher_text.as_ref().unwrap());

    let bytes = base64_decode(data.cipher_text.as_ref().unwrap());

    let nonce = base64_decode(&data.nonce);
    let nonce = GenericArray::from_slice(nonce.as_ref().unwrap());

    let salt = base64_decode(&data.salt).unwrap();

    let decrypted = decrypt_password(MASTER_PASSWORD, &salt.try_into().expect("failed to convert into [u8; 16]"), nonce, bytes.unwrap().as_ref());
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

pub fn test_crud(passwords: &mut HashMap<String, EncryptedPassword>) {
    const APP_NAME: &str = "bonjour monde";

    println!();
    let _ = create_update_password(APP_NAME, passwords, 16, true, MASTER_PASSWORD);
    _ = save_passwords_to_disk(passwords);
    _ = read_password(APP_NAME, passwords, MASTER_PASSWORD, false);
    delete_password(APP_NAME, passwords);
}
