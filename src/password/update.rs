use std::collections::HashMap;
use crate::encryption::encrypt::{encrypt_password, EncryptedPassword};

use super::create::create_random_password;

pub fn update_password(application_name: &str, password: &EncryptedPassword, passwords: &mut HashMap<String, EncryptedPassword>) {
    passwords.insert(application_name.to_string(), password.clone());
}

pub fn create_update_password(application_name: &str, passwords: &mut HashMap<String, EncryptedPassword>,
                              length: usize, include_special_characters: bool, master_password: &str)
-> Result<(), ()>
{
    let new_password = create_random_password(length, include_special_characters);
    let encrypted = encrypt_password(master_password, &new_password);

    let encrypted = match encrypted {
        Some(v) => v,
        None => {
            eprintln!("Error while generating random password to save for {}!", application_name);
            return Err(());
        }
    };

    update_password(application_name, &encrypted, passwords);
    Ok(())
}
