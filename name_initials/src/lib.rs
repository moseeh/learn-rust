pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::with_capacity(names.len());
    
    for name in names {
        let mut initials_string = String::new();
        
        for word in name.split_whitespace() {
            if let Some(initial) = word.chars().next() {
                initials_string.push(initial);
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
