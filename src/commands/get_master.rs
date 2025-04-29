use std::io::{self, Write};
use rpassword::read_password;

pub fn get_master_password() -> String {
    print!("Enter master password: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    
    read_password().expect("Failed to read password!")
}