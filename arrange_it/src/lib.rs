pub fn arrange_phrase(phrase: &str) -> String {
    // Split the phrase into words
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    
    // Sort the words based on the numbers they contain
    words.sort_by_key(|word| {
        // Find the first digit in the word
        for (i, c) in word.chars().enumerate() {
            if c.is_digit(10) {
                return word[i..].parse::<u32>().unwrap_or(0);
            }
        }
        0 // Default if no number is found
    });
    
    // Join the sorted words back into a string
    words.join(" ")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
