use crate::encryption::{decrypt::decrypt_password, encrypt::encrypt_password, encrypted_password::EncryptedPassword};
use std::{collections::HashMap, sync::atomic::{AtomicBool, Ordering}};
use rayon::prelude::*;

pub fn handle_change_master(master_password: &str, new_master_password: &str, passwords: &mut HashMap<String, EncryptedPassword>)
-> Result<(), ()>
{
    let failed = AtomicBool::new(false);

    let new_passwords: Vec<(String, EncryptedPassword)> = passwords.par_iter().filter_map(|(key, value)| {
            let typed = value.to_typed();
            let typed = match typed {
                Ok(typed) => typed,
                Err(_) => {
                    eprintln!("Error decrypting password for re-encryption!");
                    failed.store(true, Ordering::Relaxed);
                    return None;
                }
            };

            let decrypted = decrypt_password(master_password, &typed.salt, &typed.nonce, &typed.cipher_text);
            let decrypted = match decrypted {
                Ok(decrypted) => decrypted,
                Err(_) => {
                    eprintln!("Error decrypting password for re-encryption!");
                    failed.store(true, Ordering::Relaxed);
                    return None;
                }
            };

            let decrypted = match decrypted {
                Some(decrypted) => decrypted,
                None => {
                    eprintln!("Error decrypting password for re-encryption!");
                    failed.store(true, Ordering::Relaxed);
                    return None;
                }
            };

            let encrypted = encrypt_password(new_master_password, &decrypted);
            let encrypted = match encrypted {
                Some(encrypted) => encrypted,
                None => {
                    eprintln!("Error encrypting password for re-encryption!");
                    failed.store(true, Ordering::Relaxed);
                    return None;
                }
            };

            Some((key.clone(), encrypted))
        }).collect();

    for (key, value) in new_passwords.iter() {
        passwords.insert(key.clone(), value.clone());
    }

    if failed.load(Ordering::Relaxed) {
        Err(())
    }
    else {
        Ok(())
    }
}