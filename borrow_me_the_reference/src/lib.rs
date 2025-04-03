pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut skip_next = false;
    
    for (i, &c) in chars.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        match c {
            '-' => {
                // Backspace: remove the last character if possible
                result.pop();
            },
            '+' => {
                // Delete: skip the next character
                skip_next = true;
            },
            _ => {
                // Regular character
                result.push(c);
            }
        }
    }
    
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
