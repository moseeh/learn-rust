pub fn first_subword(mut s: String) -> String {
    if s.is_empty() {
        return s;
    }

    // Handle snake_case
    if let Some(pos) = s.find('_') {
        s.truncate(pos);
        return s;
    }

    // Handle camelCase and PascalCase
    let mut end_pos = 1;
    let chars: Vec<char> = s.chars().collect();
    
    // Find the position of the second capital letter or the end of the string
    while end_pos < chars.len() {
        // If current char is uppercase and previous char is lowercase,
        // we've found the beginning of the next word
        if chars[end_pos].is_uppercase() && chars[end_pos - 1].is_lowercase() {
            break;
        }
        end_pos += 1;
    }

    // Convert the result back to a String
    s.truncate(end_pos);
    s
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
