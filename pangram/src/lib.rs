use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut letters = HashSet::new();
    for ch in s.chars(){
        if ch.is_ascii_alphabetic() {
            letters.insert(ch.to_ascii_lowercase());
        }
    }
    letters.len() == 26
}