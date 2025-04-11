pub fn scytale_cipher(message: String, key: u32) -> String {
    // Interpret key as the number of columns.
    let columns = key as usize;
    let len = message.len();
    // Determine the number of rows needed (ceiling division)
    let rows = (len + columns - 1) / columns;
    // Calculate padded length (all cells in the grid)
    let padded_len = rows * columns;
    
    // Create a vector of characters from the message and pad with spaces.
    let mut padded: Vec<char> = message.chars().collect();
    padded.resize(padded_len, ' ');
    
    // Read the grid column-by-column.
    let mut result = String::with_capacity(padded_len);
    for col in 0..columns {
        for row in 0..rows {
            // The cell index is: row * columns + col.
            result.push(padded[row * columns + col]);
        }
    }
    
    // Trim any trailing whitespace that was added as padding.
    result.trim_end().to_string()
}
