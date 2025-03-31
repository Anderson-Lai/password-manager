use argon2::{Argon2, Algorithm, Version, Params};

pub fn generate_secure_key(password: &[u8], salt: &[u8; 16]) -> Option<[u8; 32]> {
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());

    let mut key = [0 as u8; 32];
    let res = argon2.hash_password_into(password, salt, &mut key);
    
    match res {
        Ok(_) => Some(key),
        Err(_) => {
            eprintln!("Error while generating secure key using KDF!");
            None
        }
    }
}
