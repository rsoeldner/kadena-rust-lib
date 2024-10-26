use crate::CryptoError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

pub fn bin_to_hex(data: &[u8]) -> String {
    hex::encode(data)
}

pub fn hex_to_bin(s: &str) -> Result<Vec<u8>, CryptoError> {
    Ok(hex::decode(s)?)
}

pub fn base64url_encode(input: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(input)
}

pub fn base64url_decode(input: &str) -> Result<Vec<u8>, CryptoError> {
    Ok(URL_SAFE_NO_PAD.decode(input)?)
}
