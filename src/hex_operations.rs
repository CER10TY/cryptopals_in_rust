use hex::FromHex;
use base64::encode;
use base64::decode;

#[allow(dead_code)]
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    Vec::from_hex(hex).expect("Invalid hex string!")
}

#[allow(dead_code)]
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    encode(bytes)
}

#[allow(dead_code)]
pub fn base64_to_bytes(b64: &str) -> Vec<u8> {
    decode(b64).unwrap()
}

#[allow(dead_code)]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let hex: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let b64: String = bytes_to_base64(&hex_to_bytes(&hex));
        assert_eq!(b64, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string());
    }
}