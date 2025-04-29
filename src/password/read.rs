use std::collections::HashMap;
use crate::{clipboard::write::write_to_clipboard, encryption::{decrypt::decrypt_password, encrypted_password::EncryptedPassword}};

pub fn read_password(application_name: &str, passwords: &mut HashMap<String, EncryptedPassword>,
                     master_password: &str, print_to_terminal: bool)
-> Result<(), ()>
{
    let stored = passwords.get(application_name);
    let stored = match stored {
        Some(v) => v,
        None => {
            eprintln!("Attempting to read password for {} when password for {} does not exist!", application_name, application_name);
            return Err(());
        }
    };

    let typed_password = stored.to_typed();
    let typed_password = match typed_password {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error occured while decrypting password for {}!", application_name);
            return Err(());
        }
    };

    let salt = typed_password.salt;
    let nonce = &typed_password.nonce;
    let cipher_text = typed_password.cipher_text;

    // decrypt stored password after proper data and type conversions
    let decrypted = decrypt_password(master_password, &salt, nonce, &cipher_text);

    // if decrypted is Ok(None) it is likely a wrong password
    match decrypted {
        Ok(option) => {
            match option {
                Some(v) => {
                    if print_to_terminal {
                        println!("Password for {}: {}", application_name, v);
                        Ok(())
                    }
                    else {
                        match write_to_clipboard(&v) {
                            Ok(_) => Ok(()),
                            Err(_) => {
                                println!("Since writing password for {} to clipboard failed, \
                                         password will be printed to terminal instead", application_name);

                                println!("Password for {}: {}", application_name, v);
                                Ok(())
                            } 
                        }
                    }
                }
                None => {
                    println!("Incorrect master password provided!");
                    Ok(())
                }
            }
        },
        Err(_) => {
            eprintln!("Error whilst converting decrypted password for {} to a string!", application_name);
            Err(())
        }
    }
}
