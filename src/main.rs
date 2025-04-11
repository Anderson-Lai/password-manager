mod encryption;
mod tests;
mod storage;
mod constants;
mod base64;
mod password;
mod clipboard;
mod platform;
use std::env;
use storage::init::init_data;
use tests::{test_crud, test_encryption_decryption, test_password_saving};

fn main() {
    let passwords = init_data();
    if let Err(_) = passwords {
        eprintln!("passwords.json could not be created; Exiting program now!");
        return;
    }
    let mut passwords = passwords.unwrap();

    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    {
        test_encryption_decryption(); 
        test_password_saving(&mut passwords);
        test_crud(&mut passwords);
    }

    if args.len() <= 1 {
        todo!("Open GUI");
    }
}

