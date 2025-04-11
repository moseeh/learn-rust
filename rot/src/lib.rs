pub fn rotate(input: &str, key: i8) -> String {
    let normalized_key = (((key % 26) + 26) % 26) as u8;
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let offset = (c as u8 - base + normalized_key) % 26;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}
