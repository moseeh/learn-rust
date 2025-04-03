pub fn delete_and_backspace(s: &mut String) {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '-' => {
                result.pop();
                i += 1;
            },
            '+' => {
                let mut count = 0;
                while i < chars.len() && chars[i] == '+' {
                    count += 1;
                    i += 1;
                }
                i += count;
            },
            c => {
                result.push(c);
                i += 1;
            }
        }
    }
    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(op_pos) = s.find(|c: char| c == '+' || c == '-') {
            let (left, right) = s.split_at(op_pos);
            let x = left.trim().parse::<i32>().unwrap_or(0);
            let y = right[1..].trim().parse::<i32>().unwrap_or(0);
            
            *s = match s.chars().nth(op_pos) {
                Some('+') => (x + y).to_string(),
                Some('-') => (x - y).to_string(),
                _ => s.clone(),
            };
        }
    }
}