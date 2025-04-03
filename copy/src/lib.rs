pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = (c.abs() as f64).ln();
    
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let mut exp_string = String::new();
    for ch in a.chars() {
        if !ch.is_digit(10) {
            continue;
        }
        if let Some(digit) = ch.to_digit(10) {
            let exp_value = (digit as f64).exp();
            exp_string.push_str(&exp_value.to_string());
            exp_string.push(' ');
        }
    }
    if !exp_string.is_empty() {
        exp_string.pop();
    }
    (a, exp_string)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_vec: Vec<f64> = b.iter()
        .map(|&x| (x.abs() as f64).ln())
        .collect();
    (b, ln_vec)
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
