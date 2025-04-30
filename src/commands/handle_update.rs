use std::collections::HashMap;
use crate::{encryption::{encrypt::encrypt_password, encrypted_password::EncryptedPassword}, password::update::{create_update_password, update_password}};

pub fn handle_update(master_password: &str, application_password: &str, application_name: &str, passwords: &mut HashMap<String, EncryptedPassword>)
-> Result<(), ()>
{
    let encrypted = encrypt_password(master_password, application_password);
    let encrypted = match encrypted {
        Some(v) => v,
        None => {
            eprintln!("Error generating password for {}!", application_name);
            return Err(());
        }
    };

    update_password(application_name, &encrypted, passwords);
    return Ok(());
}

pub fn handle_random_update(master_password: &str, application_name: &str,
    length: usize, include_special_characters: bool, passwords: &mut HashMap<String, EncryptedPassword>) -> Result<(), ()>
{
    return create_update_password(application_name, passwords, length, include_special_characters, master_password)
}