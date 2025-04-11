use std::collections::HashMap;
use crate::encryption::encrypt::EncryptedPassword;

pub fn add_password(application_name: &str, password: &EncryptedPassword, 
                    passwords: &mut HashMap<String, EncryptedPassword>, force_insert: bool)
-> Result<(), ()>
{
    if !force_insert && passwords.contains_key(application_name) {
        eprintln!("Error while adding password to password hashmap; application already contains a password!");
        return Err(());
    }

    match passwords.insert(application_name.to_string(), password.clone()) {
        Some(v) => {
            if force_insert {
                return Ok(());
            }

            eprintln!("This arm should not be reached; existing password should have already been checked for!");
            passwords.insert(application_name.to_string(), v);
            Err(())
        }
        None => {
            Ok(())
        }
    }
}

