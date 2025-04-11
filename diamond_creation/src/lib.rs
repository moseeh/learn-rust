pub fn get_diamond(c: char) -> Vec<String> {
    // Early return for single-character diamond
    if c == 'A' {
        return vec!["A".to_string()];
    }
    
    let size = (c as u8 - 'A' as u8 + 1) as usize;
    let mut result = Vec::with_capacity(2 * size - 1);
    
    // Build top half (including middle row)
    for idx in 0..size {
        result.push(create_row('A' as u8 + idx as u8, c));
    }
    
    // Build bottom half (excluding middle row)
    for idx in (0..size-1).rev() {
        result.push(create_row('A' as u8 + idx as u8, c));
    }
    
    result
}

// Helper function to create a single row of the diamond
fn create_row(letter_byte: u8, center: char) -> String {
    let letter = letter_byte as char;
    let space_before = (center as usize) - (letter as usize);
    let mut row = " ".repeat(space_before);
    
    row.push(letter);
    
    if letter != 'A' {
        let inner_spaces = 2 * ((letter as u8) - ('A' as u8)) - 1;
        row.push_str(&" ".repeat((inner_spaces + 2) as usize));
        row.push(letter);
    }
    
    row.push_str(&" ".repeat(space_before));
    row
}