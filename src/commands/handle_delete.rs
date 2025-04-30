use std::collections::HashMap;
use crate::encryption::encrypted_password::EncryptedPassword;

pub fn handle_delete(application_name: &str, passwords: &mut HashMap<String, EncryptedPassword>) -> Option<EncryptedPassword> {
    passwords.remove(application_name)
}