pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = s.chars().collect();
    
    while i < chars.len() {
        match chars[i] {
            // Backspace: remove the previous character if exists
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            },
            // Delete: skip the next character if exists
            '+' => {
                // Skip the next character (increment counter)
                i += 1;
            },
            // Regular character: add to result
            c => result.push(c),
        }
        i += 1;
    }
    
    // Replace the content of the original string with our processed result
    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for item in v.iter_mut() {
        // Find the operation character (+ or -)
        let operation = if item.contains('+') { '+' } else if item.contains('-') { '-' } else { continue };
        
        // Split the string at the operation character
        let parts: Vec<&str> = item.split(operation).collect();
        if parts.len() != 2 {
            continue; // Skip if we don't have exactly two parts
        }
        
        // Parse the numbers
        if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            // Perform the operation
            let result = match operation {
                '+' => num1 + num2,
                '-' => num1 - num2,
                _ => unreachable!(),
            };
            
            // Replace the string with the result
            *item = result.to_string();
        }
    }
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
