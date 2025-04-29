use std::collections::HashMap;
use crate::{encryption::{encrypted_password::EncryptedPassword, encrypt::encrypt_password}, password::{add::add_password, create::create_random_password}};

pub fn handle_create(passwords: &mut HashMap<String, EncryptedPassword>, application_name: &str,
    master_password: &str, application_password: &str, force_insert: bool) 
-> Result<(), ()>
{
    let encrypted = encrypt_password(master_password, application_password);
    let encrypted = match encrypted {
        Some(v) => v,
        None => {
            eprintln!("Encrypting password failed; password creation can not continue");
            return Err(());
        }
    };

    add_password(application_name, &encrypted, passwords, force_insert)
}

pub fn handle_random_create(passwords: &mut HashMap<String, EncryptedPassword>, application_name: &str,
    master_password: &str, length: usize, include_special_characters: bool, force_insert: bool) 
-> Result<(), ()>
{
    let random_password = create_random_password(length, include_special_characters);
    handle_create(passwords, application_name, master_password, &random_password, force_insert)
}