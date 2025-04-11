use rand::{distributions::Alphanumeric, Rng, thread_rng};
use crate::constants;

pub fn create_random_password(length: usize, include_special_characters: bool) -> String {
    let mut rng = thread_rng();

    if include_special_characters {
        (0..length)
            .map(|_| {
                let index = rng.gen_range(0..constants::PASSWORD_CHARACTERS.len());
                constants::PASSWORD_CHARACTERS[index] as char
            })
            .collect()
    }
    else {
        rng.sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
}
