// roman_numbers_iter/src/lib.rs

// Re-export RomanDigit so it's available publicly
pub use roman_numbers::RomanDigit;
use roman_numbers::RomanNumber as BaseRomanNumber;

/// A newtype wrapper around the Base RomanNumber to support Iterator
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub BaseRomanNumber);

impl From<u32> for RomanNumber {
    fn from(num: u32) -> Self {
        RomanNumber(BaseRomanNumber::from(num))
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        // Helper to map a RomanDigit to its numeric value
        fn digit_value(d: &RomanDigit) -> u32 {
            match d {
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
                RomanDigit::Nulla => 0,
            }
        }

        // Extract the digits vector
        let digits = &self.0.0;
        // Decode to integer with subtractive rules
        let mut value: i32 = 0;
        let mut i = 0;
        while i < digits.len() {
            let v = digit_value(&digits[i]) as i32;
            if i + 1 < digits.len() {
                let next_v = digit_value(&digits[i + 1]) as i32;
                if next_v > v {
                    value += next_v - v;
                    i += 2;
                    continue;
                }
            }
            value += v;
            i += 1;
        }
        // Convert to u32 and increment
        let next_value = (value as u32).wrapping_add(1);

        // Build the next RomanNumber
        let next_base = BaseRomanNumber::from(next_value);
        let next = RomanNumber(next_base.clone());

        // Update self and return
        *self = next.clone();
        Some(next)
    }
}
