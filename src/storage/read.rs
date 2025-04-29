use std::{collections::HashMap, fs::OpenOptions, io::Read};
use crate::{constants, encryption::encrypted_password::EncryptedPassword};

pub fn read_saved_passwords() -> Result<HashMap<String, EncryptedPassword>, ()>{
    let file = OpenOptions::new().read(true).write(true).open(constants::FILE_NAME);
    let mut file = match file {
        Ok(f) => f,
        Err(_) => return Err(())
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("Error reading stored passwords in {}!", constants::FILE_NAME);
            return Err(());
        }
    }

    // read stored passwords
    let contents = serde_json::from_slice(contents.as_bytes());
    let contents: HashMap<String, EncryptedPassword> = match contents {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error while reading stored passwords in {}!", constants::FILE_NAME);
            return Err(());
        }
    };

    Ok(contents)
}
