use std::collections::HashMap;
use aes_gcm::aead::generic_array::GenericArray;
use crate::{base64::decode::base64_decode, clipboard::write::write_to_clipboard, encryption::{decrypt::decrypt_password, encrypt::EncryptedPassword}};

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

    let salt = base64_decode(&stored.salt);
    let salt = match salt {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error occured while decrypting salt for {}; password reading for {} can not proceed!", application_name, application_name);
            return Err(());
        }
    };

    // convert the salt into a [u8; 16]
    let salt: [u8; 16] = salt.try_into().expect(format!("Failed to coerce salt into [u8; 16] for {}", application_name).as_str());

    let nonce = base64_decode(&stored.nonce);
    let nonce = match nonce {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error occured while decrypting nonce for {}: password reading for {} can not proceed!", application_name, application_name);
            return Err(());
        }
    };

    // convert nonce into a GenericArray<u8, U12>
    let nonce = GenericArray::from_slice(&nonce);

    let cipher_text = match &stored.cipher_text {
        Some(v) => v,
        None => {
            eprintln!("Ciphertext for {} was None; password reading for {} can not proceed!", application_name, application_name);
            return Err(());
        }
    };

    // decode the ciphertext from base64
    let cipher_text = base64_decode(cipher_text);
    let cipher_text = match cipher_text {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error whilst converting cyphertext from base64 for {};\
                      password reading for {} can not proceed!", application_name, application_name);
            return Err(());
        }
    };

    // convert ciphertext into Vec<u8>
    let cipher_text = cipher_text.to_vec();

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
