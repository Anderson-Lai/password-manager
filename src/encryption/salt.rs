use rand::RngCore;
use rand::rngs::OsRng;

pub fn generate_salt() -> [u8; 16] {
    let mut salt: [u8; 16] = [0 as u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}
