use base64::{engine::general_purpose, Engine};

pub fn base64_encode<T>(value: T) -> String 
where 
    T: AsRef<[u8]>
{
    general_purpose::STANDARD.encode(value)
}

