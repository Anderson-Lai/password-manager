use std::collections::HashMap;
use crate::{commands::handle_help::handle_help, encryption::encrypt::EncryptedPassword};

pub fn parse_commands(arguments: &Vec<String>, passwords: &mut HashMap<String, EncryptedPassword>) {
    let command = &arguments[1];

    if command == "create" {
        
    }
    else if command == "read" {

    }
    else if command == "update" {

    }
    else if command == "delete" {
        
    }
    else if command == "list" {

    }
    else if command == "change-master" {

    }
    else if command == "help" {
        handle_help();
    }
    else {
        eprintln!("Unknown command: {}", command);
    }
}