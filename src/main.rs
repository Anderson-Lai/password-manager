mod encryption;
mod tests;
use std::env;
use tests::test_encryption_decryption;

fn main() {
    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    {
        test_encryption_decryption(&args); 
    }

    if args.len() <= 1 {
        todo!("Open GUI");
    }
}

