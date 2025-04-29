use std::collections::HashMap;
use crate::{commands::{handle_create::{handle_create, handle_random_create}, handle_help::handle_help}, encryption::encrypt::EncryptedPassword};
use super::get_master::get_master_password;

pub fn parse_commands(arguments: &Vec<String>, passwords: &mut HashMap<String, EncryptedPassword>) -> Result<(), ()> {
    let command = &arguments[1];

    if command == "create" {
        let master_password = get_master_password();
        let application_name = &arguments[2];
        
        // determine if a random password needs to be generated
        let mut generate_password = false;
        if arguments.len() < 3 || arguments[3].chars().next().unwrap() == '-' {
            generate_password = true;
        }

        // argument parsing
        let mut length: usize = 16;
        let mut include_special_characters = false;
        let mut force_insert = false;

        let mut argument_iterator = arguments.iter().skip(3);
        while let Some(arg) = argument_iterator.next() {
            if arg == "--length" {
                length = match argument_iterator.next() {
                    Some(v) => match v.parse() {
                        Ok(v) => v,
                        Err(_) => {
                            // safe to unwrap since it is in the Some arm already
                            eprintln!("Invalid length argument: {}!", argument_iterator.next().unwrap());
                            return Err(());
                        }
                    },
                    None => {
                        eprintln!("Missing length argument!");
                        return Err(());
                    }
                };
            }
            else if arg == "-s" || arg == "--special" {
                include_special_characters = true;
            }
            else if arg == "-f" || arg == "--force" {
                force_insert = true;
            }
            else {
                eprintln!("Invalid argument: {}!", arg);
                return Err(());
            }
        }

        if generate_password {
            return handle_random_create(passwords, application_name, &master_password, length, include_special_characters, force_insert);
        }
        else {
            let application_password = &arguments[3];
            return handle_create(passwords, application_name, &master_password, application_password, force_insert);
        }
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

    eprintln!("Unknown command: {}", command);
    Err(())
}