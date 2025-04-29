use std::collections::HashMap;
use crate::encryption::{decrypt::decrypt_password, encrypted_password::EncryptedPassword};

pub fn check_master_password(master_password: &str, passwords: &mut HashMap<String, EncryptedPassword>) -> Result<bool, ()> {
    let initial_password = passwords.get("password-manager");
    let initial_password = match initial_password {
        Some(v) => v,
        None => {
            eprintln!("Intial password does not exist!");
            return Err(());
        }
    };

    let typed = initial_password.to_typed();
    let typed = match typed {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Converting to typed password failed!");
            return Err(());
        }
    };

    match decrypt_password(master_password, &typed.salt, &typed.nonce, &typed.cipher_text) {
        Ok(v) => {
            match v {
                Some(_) => Ok(true),
                None => Ok(false)
            }
        }
        Err(_) => {
            eprintln!("Checking master password failed!");
            Err(())
        }
    }
}