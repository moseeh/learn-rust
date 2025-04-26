// roman_numbers_iter/src/lib.rs

// Re-export RomanDigit so it’s available publicly
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

        // Extract the inner digits vector
        let digits = &self.0.0;
        // Compute the integer value, accounting for subtractive notation
        let mut value = 0;
        for i in 0..digits.len() {
            let v = digit_value(&digits[i]);
            if i + 1 < digits.len() && digit_value(&digits[i + 1]) > v {
                value -= v;
            } else {
                value += v;
            }
        }

        // Increment and build the next RomanNumber
        let next_value = value + 1;
        let next_base = BaseRomanNumber::from(next_value);
        let next = RomanNumber(next_base.clone());

        // Update self and return the new value
        *self = next.clone();
        Some(next)
    }
}
