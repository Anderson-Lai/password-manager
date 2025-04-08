use std::{collections::HashMap, fs::OpenOptions, io::Write, path::Path};
use crate::{constants, encryption::encrypt::EncryptedPassword};
use super::read::read_saved_passwords;

pub fn init_data() -> Result<HashMap<String, EncryptedPassword>, ()> {

    let path = Path::new(constants::FILE_NAME);
    if !path.exists() {
        let file = OpenOptions::new().create(true).write(true).open(constants::FILE_NAME);
        match file {
            Ok(mut f) => {
                match f.write_all(b"{}") {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("Error while writing {{}} to {} during file creation!", constants::FILE_NAME);
                        return Err(());
                    }
                }
            },
            Err(_) => {
                eprintln!("Error while creating {} to store passwords!", constants::FILE_NAME);
                return Err(());
            }
        }
    };

    let contents = read_saved_passwords();
    match contents {
        Ok(v) => Ok(v),
        Err(_) => {
            eprintln!("Error while reading saved passwords from {}", constants::FILE_NAME);
            Err(())
        }
    }
}
