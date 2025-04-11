fn scytale_cipher(message: String, i: u32) -> String {
    // Check for empty message
    if message.is_empty() {
        return String::new();
    }

    // Convert i to usize for safe indexing
    let cols = i as usize;

    // Calculate rows
    let message_len = message.len();
    let rows = (message_len + cols - 1) / cols; // Ceiling division

    // Create a buffer for the result
    let mut result = String::with_capacity(message_len);

    // Convert message to a vector of chars for easier access
    let chars: Vec<char> = message.chars().collect();

    // Read column by column
    for col in 0..cols {
        for row in 0..rows {
            let index = row * cols + col;
            if index < message_len {
                result.push(chars[index]);
            }
        }
    }

    result
}
