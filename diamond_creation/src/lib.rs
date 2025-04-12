pub fn get_diamond(c: char) -> Vec<String> {
    // Early return for single-character diamond
    if c == 'A' {
        return vec!["A".to_string()];
    }
    
    let mut result = Vec::new();
    let mut space_after = 0;
    
    // Build top half (including middle row)
    for letter in 'A'..=c {
        let mut row = String::new();
        let space_before = c as usize - letter as usize;
        
        row.push_str(&" ".repeat(space_before));
        row.push(letter);
        
        if letter != 'A' { 
            space_after += 1;
            row.push_str(&" ".repeat(space_after));
            row.push_str(&" ".repeat(space_after - 1));
            row.push(letter);
        }
        
        row.push_str(&" ".repeat(space_before));
        result.push(row);
    }
    
    // Build bottom half (excluding middle row)
    for letter in ('A'..c).rev() {
        let mut row = String::new();
        let space_before = c as usize - letter as usize;
        
        row.push_str(&" ".repeat(space_before));
        row.push(letter);
        
        if letter != 'A' { 
            space_after -= 1;
            row.push_str(&" ".repeat(space_after));
            row.push_str(&" ".repeat(space_after - 1));
            row.push(letter);
        }
        
        row.push_str(&" ".repeat(space_before));
        result.push(row);
    }
    
    result
}