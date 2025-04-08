mod encryption;
mod tests;
mod storage;
mod constants;
use std::env;
use storage::init::init_file;
use tests::test_encryption_decryption;

fn main() {
    let file = init_file();
    if let Err(_) = file {
        eprintln!("passwords.json could not be created; Exiting program now!");
        return;
    }

    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    {
        test_encryption_decryption(&args); 
    }

    if args.len() <= 1 {
        todo!("Open GUI");
    }
}

