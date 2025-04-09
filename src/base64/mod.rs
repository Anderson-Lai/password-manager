use base64::{engine::general_purpose, Engine};

pub fn base64_encode<T>(value: T) -> String 
where 
    T: AsRef<[u8]>
{
    general_purpose::STANDARD.encode(value)
}

pub fn base64_decode<T>(base64: T) -> Result<Vec<u8>, base64::DecodeError>
where 
    T: AsRef<[u8]>
{
    general_purpose::STANDARD.decode(base64)
}
