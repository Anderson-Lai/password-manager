use std::collections::HashMap;
use crate::{commands::{handle_create::{handle_create, handle_random_create}, handle_help::handle_help, handle_list::handle_list, handle_read::handle_read}, encryption::encrypt::EncryptedPassword};
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
        if !generate_password {
            argument_iterator = arguments.iter().skip(4);
        }

        while let Some(arg) = argument_iterator.next() {
            if arg == "--length" {
                length = match argument_iterator.next() {
                    Some(v) => match v.parse() {
                        Ok(value) => value,
                        Err(_) => {
                            eprintln!("Invalid length argument: {}!", v);
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
                // since multiple flags may be put together, like in '-sf'
                for letter in arg.chars().skip(1) {
                    if letter == 's' {
                        include_special_characters = true;
                    }
                    else if letter == 'f' {
                        force_insert = true;
                    }
                    else {
                        eprintln!("Unknown flag: {}!", letter);
                        return Err(());
                    }
                }
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
        let master_password = get_master_password();

        // argument parsing
        let mut print_to_terminal = false;
        if arguments.len() > 3 {
            if arguments[3] == "-p" || arguments[3] == "--print" {
                print_to_terminal = true;
            }
            else {
                eprintln!("Unknown flag: {}!", arguments[3]);
                return Err(());
            }
        }

        return handle_read(&master_password, &arguments[2], passwords, print_to_terminal);
    }
    else if command == "update" {

    }
    else if command == "delete" {
        
    }
    else if command == "list" {
        handle_list(passwords);
        return Ok(());
    }
    else if command == "change-master" {

    }
    else if command == "help" {
        handle_help();
        return Ok(());
    }

    eprintln!("Unknown command: {}", command);
    Err(())
}