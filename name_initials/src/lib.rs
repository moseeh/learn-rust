pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::with_capacity(names.len());
    
    for name in names {
        let mut initials_string = String::new();
        let words: Vec<&str> = name.split_whitespace().collect();
        
        for (i, word) in words.iter().enumerate() {
            if let Some(initial) = word.chars().next() {
                initials_string.push(initial);
                initials_string.push('.');

                if i < words.len() - 1 {
                    initials_string.push(' ');
                }
            }
        }
        result.push(initials_string);
    }
    
    result
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
