pub fn scytale_cipher(message: String, size: u32) -> String {
    let columns = size as usize;
    let len = message.len();
    // Determine the required number of rows (ceiling division)
    let rows = (len + columns - 1) / columns;
    let chars: Vec<char> = message.chars().collect();
    
    let mut result = String::new();
    // Read the grid column-by-column.
    for col in 0..columns {
        for row in 0..rows {
            let index = row * columns + col;
            if index < len {
                result.push(chars[index]);
            }
        }
    }
    
    result
}
