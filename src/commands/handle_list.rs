use std::collections::HashMap;
use crate::encryption::encrypt::EncryptedPassword;

pub fn handle_list(passwords: &mut HashMap<String, EncryptedPassword>) {
    for key in passwords.keys() {
        println!("{}", key);
    }
}