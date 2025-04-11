pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    let len = message.len();
    let chars: Vec<char> = message.chars().collect();
    
    // Calculate the number of columns required
    let cols = (len + i - 1) / i; // ceiling division to get enough columns

    // Pad message if necessary
    let padded_len = i * cols;
    let mut padded_chars = chars.clone();
    padded_chars.resize(padded_len, ' '); // fill with spaces

    // Build the ciphered string by reading column-wise
    let mut result = String::with_capacity(padded_len);
    for col in 0..cols {
        for row in 0..i {
            result.push(padded_chars[row * cols + col]);
        }
    }

    result
}
