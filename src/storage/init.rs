use std::{collections::HashMap, fs::OpenOptions, io::Write, path::Path};
use crate::{commands::{get_master::get_master_password, handle_create::handle_random_create}, constants, encryption::encrypt::EncryptedPassword};
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
    let mut contents = match contents {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error while reading saved passwords from {}", constants::FILE_NAME);
            return Err(());
        }
    };

    // an already hashed password under the application name 'password-manager' is
    // needed for some commands
    if !contents.contains_key("password-manager") {
        println!("Please enter your master password to initialize the password manager.");
        let master_password = get_master_password();
        match handle_random_create(&mut contents, "password-manager", &master_password, 20, true, false) {
            Ok(_) => Ok(contents),
            Err(_) => {
                eprintln!("Error during intializing password creation!");
                Err(())
            }
        }
    }
    else {
        Ok(contents)
    }
}
