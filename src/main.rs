mod xor_operations;
mod hex_operations;
use std::collections::BTreeMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    crate::xor_operations::break_repeating_key_xor(2, 4, &[20, 40, 55, 22, 12, 16, 21, 78])
}

pub fn score_plaintext(bytes: &[u8]) -> f32 {
    let mut plaintext_score: f32 = 0.0;

    // Reference: https://www3.nd.edu/~busiforc/handouts/cryptography/Letter%20Frequencies.html
    let english_character_frequency = BTreeMap::from([
        // The space is a bit of a cheat, as it's not a legitimate character per se and certainly not at the top of the list
        (" ", 0.13),
        ("e", 0.12702),
        ("t", 0.09056),
        ("a", 0.08167),
        ("o", 0.07507),
        ("i", 0.06966),
        ("n", 0.06749),
        ("s", 0.06327),
        ("h", 0.06094),
        ("r", 0.05987),
        ("d", 0.04253),
        ("l", 0.04025),
        ("c", 0.02782),
        ("u", 0.02758), // Rip ETAOIN SHRDLU
        ("m", 0.02406),
        ("w", 0.02360),
        ("f", 0.02228),
        ("g", 0.02015),
        ("y", 0.01974),
        ("p", 0.01929),
        ("b", 0.01492),
        ("v", 0.00978),
        ("k", 0.00772),
        ("j", 0.00153),
        ("x", 0.00150),
        ("q", 0.00095),
        ("z", 0.00074)
    ]);
    
    // For the Cryptopals challenges, only ASCII characters are used (in the legible text anyway)
    // However, chars() uses unicode _points_ instead of full _characters_, so working with Grapheme Clusters is usually a safer bet in regards to UTF-8 compatibility
    
    // Also, we clone the Vec<u8> as working with the reference does not work
    let ciphertext = String::from_utf8(bytes.to_owned());

    // .unwrap() is unsafe to call as not all strings given to this function are actually valid UTF-8
    match ciphertext {
        Ok(v) => {
            for c in v.graphemes(true) {
                // Incredibly important but slightly cumbersome to make this all _lowercase_. Specifically, challenge 3 features a majority of upper case characters that are NOT rated
                match english_character_frequency.get(&c.to_ascii_lowercase().as_str()) {
                    Some(score) => plaintext_score += score,
                    None => {}
                }
            }
        
        },
        Err(_) => {}
    };

    plaintext_score
}

// The Hamming Distance is the number of differing _bits_, not bytes!, between two strings
// This only works with equal-length Vectors
pub fn calculate_hamming_distance(a: &Vec<u8>, b: &Vec<u8>) -> u64 {
    // Looks complicated, but it's not really
    // First, we make an iterator out of a, zipping it up to b. This creates (a, tuple), where a[0] is the first element, and b[0] the second and so on
    // Fold() is where magic happens. It iterates through the whole tuple just created and returns a single value
    // 0 is the initial value, a holds the _result_ of the previous rounds (so 0 initially), and (b, c) are the respective tuple values
    // And finally, we just XOR the actual tuple values and count the numbers of 1 bits, indicating the Hamming distance
    a.iter().zip(b).fold(0, |a, (b, c)| a + (*b ^ *c).count_ones() as u64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_hamming_distance() {
        let a: Vec<u8> = String::as_bytes(&"this is a test".to_string()).to_vec();
        let b: Vec<u8> = String::as_bytes(&"wokka wokka!!!".to_string()).to_vec();

        let distance: u64 = crate::calculate_hamming_distance(&a, &b);

        assert_eq!(distance, 37);
    }
}
