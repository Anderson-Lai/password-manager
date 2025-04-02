use std::env;

use base64::{engine::general_purpose, Engine};
use encryption::{decrypt::decrypt_password, encrypt::generate_ciphertext, kdf::generate_secure_key, nonce::generate_nonce, salt::generate_salt};
mod encryption;

fn main() {
    let _args: Vec<String> = env::args().collect();

/*
    if args.len() <= 1 {
        todo!("Open GUI");
    }
*/
    
    #[cfg(debug_assertions)]
    {
        debug_main(); 
        return;
    }

}

fn debug_main() {
    const MASTER_PASSWORD: &str = "happy birthday";
    const APPLICATION_PASSWORD: &str = "hello123!";

    let salt = generate_salt();
    let secure_key = generate_secure_key(MASTER_PASSWORD.as_bytes(), &salt);
    let nonce = generate_nonce();

    let encrypted = generate_ciphertext(APPLICATION_PASSWORD, &secure_key.unwrap(), &nonce);
    let encrypted = encrypted.unwrap();

    let bytes = general_purpose::STANDARD.decode(&encrypted);

    println!("{}", encrypted);
    let decrypted = decrypt_password(MASTER_PASSWORD, &salt, &nonce, bytes.unwrap().as_ref());

    let decrypted = decrypted.unwrap().unwrap();
    println!("{}", decrypted);

    assert!(APPLICATION_PASSWORD == decrypted);
}
