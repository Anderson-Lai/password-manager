use std::collections::HashMap;
use crate::encryption::encrypt::EncryptedPassword;

pub fn delete_password(application_name: &str, passwords: &mut HashMap<String, EncryptedPassword>) {
    match passwords.remove(application_name) {
        Some(_) => {
            println!("Password for {} was successfully removed!", application_name);
        }
        None => {
            println!("Password for {} never existed; removing password does nothing!", application_name);
        }
    }
}
