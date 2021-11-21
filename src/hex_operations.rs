use hex::FromHex;
use base64::encode;

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    Vec::from_hex(hex).expect("Invalid hex string!")
}

pub fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    encode(bytes)
}

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
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