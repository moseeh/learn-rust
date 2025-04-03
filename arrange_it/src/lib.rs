pub fn arrange_phrase(phrase: &str) -> String {
    // Parse each word to extract position and clean word
    let mut words: Vec<(String, usize)> = phrase
        .split_whitespace()
        .map(|word| {
            let mut position = 0;
            let mut clean_word = String::new();
            
            for c in word.chars() {
                if c.is_digit(10) {
                    position = position * 10 + c.to_digit(10).unwrap() as usize;
                } else {
                    clean_word.push(c);
                }
            }
            
            (clean_word, position)
        })
        .collect();
    
    // Sort words by their position
    words.sort_by_key(|(_, pos)| *pos);
    
    // Join sorted words
    words.into_iter()
        .map(|(word, _)| word)
        .collect::<Vec<String>>()
        .join(" ")
}