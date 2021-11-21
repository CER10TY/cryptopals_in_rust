mod xor;
mod hex_operations;
use std::collections::BTreeMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Hello World!");
}

pub fn score_plaintext(bytes: Vec<u8>) -> f32 {
    let mut plaintext_score: f32 = 0.0;

    // Reference: https://www3.nd.edu/~busiforc/handouts/cryptography/Letter%20Frequencies.html
    let english_character_frequency = BTreeMap::from([
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
        ("u", 0.02791), // Rip ETAOIN SHRDLU
        ("c", 0.02782),
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
    

    let ciphertext = String::from_utf8(bytes).unwrap();
    for c in ciphertext.graphemes(true) {
        match english_character_frequency.get(c) {
            Some(score) => plaintext_score += score,
            None => {}
        }
    }

    plaintext_score
}