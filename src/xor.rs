use std::collections::BTreeMap;

// This is the fixed XOR function, where two equal-length buffers are XORed against each other char-by-char to return a new byte combination
pub fn xor_each(byte_one: &Vec<u8>, byte_two: &Vec<u8>) -> Vec<u8> {
    let mut xor_ed: Vec<u8> = Vec::new();

    for (index, _) in byte_one.iter().enumerate() {
        xor_ed.push(byte_one[index] ^ byte_two[index]);
    }

    xor_ed
}

// Here, instead, we use a single character out of the Vec<u8> to act as the key for the bit-wise XOR operation
pub fn xor_single_character(bytes: &Vec<u8>, key: &u8) -> Vec<u8> {
    let mut xor_ed: Vec<u8> = Vec::new();

    for (index, _) in bytes.iter().enumerate() {
        xor_ed.push(bytes[index] ^ key)
    }

    xor_ed
}

pub fn score_single_byte_xor_cipher(bytes: &Vec<u8>) -> BTreeMap<Vec<u8>, f32> {
    let mut score_map: BTreeMap<Vec<u8>, f32> = BTreeMap::new();

    for byte in bytes.iter() {
        let new_plaintext: Vec<u8> = xor_single_character(&bytes, &byte);
        let score: f32 = crate::score_plaintext(new_plaintext);
        score_map.insert(xor_single_character(&bytes, &byte), score);
    }

    score_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let byte_one: Vec<u8> = crate::hex_operations::hex_to_bytes("1c0111001f010100061a024b53535009181c");
        let byte_two: Vec<u8> = crate::hex_operations::hex_to_bytes("686974207468652062756c6c277320657965");

        let xor: Vec<u8> = xor_each(&byte_one, &byte_two);

        let xor_hex: String = crate::hex_operations::bytes_to_hex(&xor);
        assert_eq!(xor_hex, "746865206b696420646f6e277420706c6179".to_string());
    }

    #[test]
    fn test_single_byte_xor_cipher() {
        let bytes: Vec<u8> = crate::hex_operations::hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let map: BTreeMap<Vec<u8>, f32> = score_single_byte_xor_cipher(&bytes);

        let mut top_two: Vec<&Vec<u8>> = Vec::new();
        let previous_score: f32 = 0.0;
        for (vec, score) in map.iter() {
            if *score == 0.0_f32 {
                continue;
            }

            if *score >= previous_score {
                top_two.insert(0, &vec);
            } else {
                top_two.push(&vec);
            }
        }
        
        assert_eq!(String::from_utf8_lossy(top_two[0]), "dHHLNI@\u{7}jd\u{0}T\u{7}KNLB\u{7}F\u{7}WHRIC\u{7}HA\u{7}EFDHI");

        // Here, the shortcomings of "simple" character frequency scoring shine, as the most legible text ends up on second place because it uses '
        assert_eq!(String::from_utf8_lossy(top_two[1]), "cOOKING\u{0}mc\u{7}S\u{0}LIKE\u{0}A\u{0}POUND\u{0}OF\u{0}BACON");
    }
}