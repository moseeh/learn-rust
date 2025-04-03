pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    // Calculate the exponential function (e^c)
    let exp = (c as f64).exp();
    
    // Calculate the natural logarithm of the absolute value
    let ln = (c.abs() as f64).ln();
    
    // Return tuple with original value, exponential, and natural log
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    // Create a new string to store the exponential values
    let mut exp_string = String::new();
    
    // Iterate through each character in the original string
    for ch in a.chars() {
        // Convert character to u32 (unicode code point)
        let code_point = ch as u32;
        
        // Calculate exponential function of the code point
        let exp_value = (code_point as f64).exp();
        
        // Add the exponential value to the new string
        exp_string.push_str(&exp_value.to_string());
        
        // Add a separator (except after the last character)
        exp_string.push(' ');
    }
    
    // Remove the trailing space if the string is not empty
    if !exp_string.is_empty() {
        exp_string.pop();
    }
    
    // Return tuple with original string and exponential string
    (a, exp_string)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    // Create a new vector to store the natural logarithm values
    let ln_vec: Vec<f64> = b.iter()
        .map(|&x| (x.abs() as f64).ln())
        .collect();
    
    // Return tuple with original vector and natural log vector
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
