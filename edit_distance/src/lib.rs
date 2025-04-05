pub fn edit_distance(source: &str, target: &str) -> usize {
    // Convert strings to character vectors for easier indexing
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    
    let source_len = source_chars.len();
    let target_len = target_chars.len();
    

    let mut matrix = vec![vec![0; target_len + 1]; source_len + 1];
    
    // Initialize first row and column
    for i in 0..=source_len {
        matrix[i][0] = i;
    }
    
    for j in 0..=target_len {
        matrix[0][j] = j;
    }
    
    // Fill the matrix
    for i in 1..=source_len {
        for j in 1..=target_len {
            // If characters match, no operation needed
            if source_chars[i - 1] == target_chars[j - 1] {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = 1 + std::cmp::min(
                    matrix[i - 1][j],
                    std::cmp::min(
                        matrix[i][j - 1],
                        matrix[i - 1][j - 1]
                    )
                );
            }
        }
    }
    
    // Return edit distance
    matrix[source_len][target_len]
}