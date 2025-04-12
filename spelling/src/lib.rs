pub fn spell(n: u64) -> String {
    // Special case for one million.
    if n == 1_000_000 {
        return "one million".to_string();
    }
    
    // For numbers 1000 or greater but less than one million,
    // split the number into a thousands and a remainder part.
    if n >= 1000 {
        let thousands = n / 1000;
        let remainder = n % 1000;
        // Build the string: thousands part + " thousand"
        // If remainder is not zero, append its spelling.
        if remainder != 0 {
            format!("{} thousand {}", spell_under_1000(thousands), spell_under_1000(remainder))
        } else {
            format!("{} thousand", spell_under_1000(thousands))
        }
    } else {
        // For numbers below 1000, handle directly.
        spell_under_1000(n)
    }
}

/// Helper function to spell any number less than 1000.
fn spell_under_1000(n: u64) -> String {
    let below_20 = [
        "zero", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty",
        "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    
    let mut result = String::new();
    
    if n < 20 {
        // For numbers less than 20, a simple lookup works.
        result.push_str(below_20[n as usize]);
    } else if n < 100 {
        // For two-digit numbers 20..=99, get the tens word.
        let t = n / 10;
        let remainder = n % 10;
        result.push_str(tens[t as usize]);
        if remainder != 0 {
            // Use a hyphen between tens and units.
            result.push('-');
            result.push_str(below_20[remainder as usize]);
        }
    } else {
        // For three-digit numbers, determine the hundreds place.
        let h = n / 100;
        let remainder = n % 100;
        result.push_str(below_20[h as usize]);
        result.push_str(" hundred");
        if remainder != 0 {
            // For any remainder, add a space and spell the remainder.
            result.push(' ');
            result.push_str(&spell_under_1000(remainder));
        }
    }
    
    result
}