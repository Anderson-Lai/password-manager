mod encryption;
mod tests;
mod storage;
mod constants;
mod base64;
mod password;
mod clipboard;
mod platform;
mod commands;
use std::env;
use commands::parse::parse_commands;
use storage::{init::init_data, save::save_passwords_to_disk};
// use tests::{test_crud, test_encryption_decryption, test_password_saving};

fn main() {
    let passwords = init_data();
    let mut passwords = match passwords {
        Ok(v) => v,
        Err(_) => {
            eprintln!("passwords.json could not be created; Exiting program now!");
            return;
        }
    };

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        todo!("Open GUI");
    }

    match parse_commands(&args, &mut passwords) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("Error while parsing commands!");
            eprintln!("Changes will not be saved!");
            eprintln!("Exiting now program now!");
            return;
        }
    }

    // #[cfg(debug_assertions)]
    // {
    //     test_encryption_decryption(); 
    //     test_password_saving(&mut passwords);
    //     test_crud(&mut passwords);
    // }

    match save_passwords_to_disk(&mut passwords) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("Error while saving passwords to disk!");
        }
    }
}