use aes_gcm::aead::{consts::U12, generic_array::GenericArray};
use serde::{Deserialize, Serialize};
use crate::base64::{decode::base64_decode, encode::base64_encode};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncryptedPassword {
    pub salt: String, // was a [u8; 16]
    pub nonce: String,
    pub cipher_text: Option<String>
}

impl EncryptedPassword {
    pub fn new(salt: &[u8; 16], nonce: &GenericArray<u8, U12>, cipher_text: Option<String>) -> Self {
        EncryptedPassword {
            salt: base64_encode(salt), 
            nonce: base64_encode(nonce), 
            cipher_text
        }
    }

    pub fn to_typed(&self) -> Result<TypedEncryptedPassword, ()> {
        let salt = base64_decode(&self.salt);
        let salt = match salt {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error occured while decrypting salt for!");
                return Err(());
            }
        };

        // convert the salt into a [u8; 16]
        let salt: [u8; 16] = salt.try_into().expect("Failed to coerce salt into [u8; 16]");

        let nonce = base64_decode(&self.nonce);
        let nonce = match nonce {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error occured while decrypting nonce!");
                return Err(());
            }
        };

        // convert nonce into a GenericArray<u8, U12>
        let nonce = GenericArray::from_slice(&nonce);

        let cipher_text = match &self.cipher_text {
            Some(v) => v,
            None => {
                eprintln!("Ciphertext for was None!");
                return Err(());
            }
        };

        // decode the ciphertext from base64
        let cipher_text = base64_decode(cipher_text);
        let cipher_text = match cipher_text {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error whilst converting cyphertext from base64 for!");
                return Err(());
            }
        };

        // convert ciphertext into Vec<u8>
        let cipher_text = cipher_text.to_vec();

        Ok (TypedEncryptedPassword {
            salt,
            nonce: nonce.clone(),
            cipher_text
        })
    }
}

pub struct TypedEncryptedPassword {
    pub salt: [u8; 16],
    pub nonce: GenericArray<u8, U12>,
    pub cipher_text: Vec<u8>
}