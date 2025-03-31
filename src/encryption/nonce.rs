use aes_gcm::aead::consts::U12;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::Nonce;
use rand::RngCore;
use rand::rngs::OsRng;

pub fn generate_nonce() -> GenericArray<u8, U12> {
    // for AES-GCM, the nonce is always **12** bytes (12 u8's)
    let mut nonce_bytes = [0 as u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes).clone();
    nonce
}
