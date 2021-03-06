// This is the fixed XOR function, where two equal-length buffers are XORed against each other char-by-char to return a new byte combination
#[allow(dead_code)]
pub fn xor_each(byte_one: &[u8], byte_two: &[u8]) -> Vec<u8> {
    let mut xor_ed: Vec<u8> = Vec::new();

    for (index, _) in byte_one.iter().enumerate() {
        xor_ed.push(byte_one[index] ^ byte_two[index]);
    }

    xor_ed
}

// Here, instead, we use a single character out of the Vec<u8> to act as the key for the bit-wise XOR operation
#[allow(dead_code)]
pub fn xor_single_character(bytes: &[u8], key: &u8) -> Vec<u8> {
    let mut xor_ed: Vec<u8> = Vec::new();

    for (index, _) in bytes.iter().enumerate() {
        xor_ed.push(bytes[index] ^ key)
    }

    xor_ed
}

#[allow(dead_code)]
pub fn score_single_byte_xor_cipher(bytes: &[u8]) -> Vec<u8> {
    let mut best_scoring_vec: Vec<u8> = Vec::new();
    let mut high_score: f32 = 0.0;

    for byte in bytes.iter() {
        let new_plaintext: Vec<u8> = xor_single_character(&bytes, &byte);
        let score: f32 = crate::score_plaintext(&new_plaintext);
        if score > 0.0 {
            if score >= high_score {
                best_scoring_vec = new_plaintext;
                high_score = score;
                //println!("{:?}, Score: {:?}", String::from_utf8_lossy(&best_scoring_vec), high_score);
            }
        }
    }

    best_scoring_vec
}

#[allow(dead_code)]
pub fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut encrypted: Vec<u8> = Vec::new();

    let key_length: usize = key.len();

    for (index, _) in bytes.iter().enumerate() {
        // Modulo iterates perfectly through the key - if Index == Key Length, Index % Key Length = 0
        let key_byte: u8 = key[index % key_length];
        encrypted.push(bytes[index] ^ key_byte);
    }

    encrypted
}

#[allow(dead_code)]
pub fn break_repeating_key_xor(min_key_size: i32, max_key_size: i32, byteset: &[u8]) {
    // chunks(2) splits up the _entire_ byteset into chunks of 2 u8. take(2) then takes 2 of these chunks and, together with collect(), returns it as &[u8]
    let chunks: Vec<&[u8]> = byteset.chunks(2).take(2).collect();
    println!("{:?}", byteset);
    println!("{:?}", chunks);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};


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
        let best_scoring_vec: Vec<u8> = score_single_byte_xor_cipher(&bytes);
        
        // The "valid" string was found by outputting a list of all strings with a score of > 0.0, and then manually looking at the most legible string
        // So really, this is just a way to encapsulate Challenge 3 and make it look good, but in a real scenario this would be manual work
        assert_eq!(String::from_utf8_lossy(&best_scoring_vec), "cOOKING\u{0}mc\u{7}S\u{0}LIKE\u{0}A\u{0}POUND\u{0}OF\u{0}BACON");
    }

    #[test]
    fn detect_single_byte_xor_cipher() {
        let file = File::open("challenge-4-strings.txt").unwrap();
        let reader = BufReader::new(file);

        let mut best_scorers: Vec<Vec<u8>> = Vec::new();

        // Each string has been single-byte XORed itself, so we get the best scorer out of each of them
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            let bytes: Vec<u8> = crate::hex_operations::hex_to_bytes(line.as_str());
            let best: Vec<u8> = score_single_byte_xor_cipher(&bytes);
            best_scorers.push(best);
        }

        // Retain only elements with any bytes in the first place
        best_scorers.retain(|x| x.len() > 0);

        // Now we've got to go through each element and rate the plain text
        let mut best_score: &Vec<u8> = &Vec::new();
        let mut high_score: f32 = 0.0;
        for scorer in best_scorers.iter() {
            let score = crate::score_plaintext(&scorer);
            if score > 0.0 {
                if score >= high_score {
                    best_score = scorer;
                    high_score = score;
                }
            }
        }

        // And finally, the best looking string gets put into assert_eq to make the test look good
        assert_eq!(String::from_utf8_lossy(&best_score), "nOW\u{0}THAT\u{0}THE\u{0}PARTY\u{0}IS\u{0}JUMPING*");
    }

    #[test]
    fn test_repeating_key_xor() {
        // Have to make sure that \n is put in manually, since otherwise \r or something might sneak in
        let stanza: Vec<u8> = String::as_bytes(&"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string()).to_vec();
        let key: Vec<u8> = String::as_bytes(&"ICE".to_string()).to_vec();

        let encrypted: Vec<u8> = repeating_key_xor(&stanza, &key);

        assert_eq!(crate::hex_operations::bytes_to_hex(&encrypted), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}