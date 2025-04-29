use std::collections::HashMap;
use crate::{encryption::encrypt::EncryptedPassword, password::read::read_password};

pub fn handle_read(master_password: &str, application_name: &str, 
    passwords: &mut HashMap<String, EncryptedPassword>, print_to_terminal: bool) 
-> Result<(), ()> 
{
    read_password(application_name, passwords, master_password, print_to_terminal)
}