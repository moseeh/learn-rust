pub fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            // Check if the word begins with a vowel
            let first_char = word.chars().next().unwrap_or('\0');
            if is_vowel(first_char) {
                // Rule 1: If word begins with a vowel, just add "ay" to the end
                return format!("{}ay", word);
            } else {
                // Check if the word starts with a consonant followed by "qu"
                if word.len() >= 3 {
                    let chars: Vec<char> = word.chars().collect();
                    if chars.len() >= 3 && chars[1] == 'q' && chars[2] == 'u' {
                        // Rule 3: Move consonant+"qu" to the end and add "ay"
                        return format!("{}{}ay", &word[3..], &word[0..3]);
                    }
                }

                // Rule 2: Move all consonants before the first vowel to the end and add "ay"
                let mut prefix_end = 0;
                for (i, c) in word.chars().enumerate() {
                    if is_vowel(c) {
                        prefix_end = i;
                        break;
                    }
                }

                if prefix_end > 0 {
                    format!("{}{}ay", &word[prefix_end..], &word[0..prefix_end])
                } else {
                    // No vowels found, just add "ay"
                    format!("{}ay", word)
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// Helper function to check if a character is a Latin vowel (aeiou)
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
