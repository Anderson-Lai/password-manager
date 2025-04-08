use std::{collections::HashMap, fs::OpenOptions, io::Write};
use crate::{constants::{self, FILE_NAME}, encryption::encrypt::EncryptedPassword};
use super::read::read_saved_passwords;

pub fn init_data() -> Result<HashMap<String, EncryptedPassword>, ()> {

    let file = OpenOptions::new().create(true).write(true).open(FILE_NAME);
    match file {
        Ok(mut f) => {
            match f.write_all(b"{}") {
                Ok(_) => {},
                Err(_) => {
                    eprintln!("Error while writing {{}} to {}!", constants::FILE_NAME);
                    return Err(());
                }
            }

            let contents = read_saved_passwords();
            match contents {
                Ok(v) => Ok(v),
                Err(_) => return Err(())
            }
        } 
        Err(_) => Err(())
    }
}
