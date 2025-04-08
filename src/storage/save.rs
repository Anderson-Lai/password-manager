use std::{collections::HashMap, fs::OpenOptions, io::Write};
use crate::{constants, encryption::encrypt::EncryptedPassword};
use super::read::read_saved_passwords;

pub fn save_passwords_to_disk(passwords: &mut HashMap<String, EncryptedPassword>) -> Result<(), ()> {

    let contents = read_saved_passwords();
    let contents = match contents {
        Ok(v) => v,
        Err(_) => return Err(())
    };

    // update current passwords only if it is not already in the current passwords hashmap
    // the current passwords hashmap may have updated passwords
    for (key, value) in &contents {
        if !passwords.contains_key(key) {
            passwords.insert(key.clone(), value.clone());
        }
    }

    // write all the new passwords to disk
    let all_passwords = serde_json::to_string_pretty(&passwords);
    match all_passwords {
        Ok(v) => {
            let file = OpenOptions::new().read(true).write(true).open(constants::FILE_NAME);
            let mut file = match file {
                Ok(f) => f,
                Err(_) => {
                    eprintln!("Error while opening {} to store passwords!", constants::FILE_NAME);
                    return Err(());
                }
            };

            match file.write_all(v.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => {
                    eprintln!("Error while writing all passwords to {}!", constants::FILE_NAME);
                    Err(())
                }
            }
        }        
        Err(_) => {
            eprintln!("Error while writnig all passwords to {}!", constants::FILE_NAME);
            Err(())
        }
    }
}
