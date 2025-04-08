use std::fs::OpenOptions;
use crate::constants::FILE_NAME;

pub fn init_file() -> Result<(), std::io::Error> {

    let res = OpenOptions::new().create(true).write(true).open(FILE_NAME);
    match res {
        Ok(_) => Ok(()),
        Err(v) => Err(v)
    }
}
