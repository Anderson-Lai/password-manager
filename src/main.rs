use std::env;
use base64::{engine::general_purpose, Engine};
use encryption::{decrypt::decrypt_password, encrypt::encrypt_password};
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

    let data = encrypt_password(MASTER_PASSWORD, APPLICATION_PASSWORD); 
    let data = data.unwrap();
    println!("{}", data.cipher_text.as_ref().unwrap());

    let bytes = general_purpose::STANDARD.decode(data.cipher_text.as_ref().unwrap());

    let decrypted = decrypt_password(MASTER_PASSWORD, &data.salt, &data.nonce, bytes.unwrap().as_ref());
    let decrypted = decrypted.unwrap().unwrap();
    println!("{}", decrypted);

    assert!(APPLICATION_PASSWORD == decrypted);
}
