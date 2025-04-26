/// roman_numbers_iterator/src/lib.rs
// Re-export the RomanNumber type from the roman_numbers crate
pub use roman_numbers::RomanNumber;

/// Implement Iterator for RomanNumber so that each call to `next` advances the value by 1.
impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        // Clone and convert current RomanNumber into its integer value
        let current_value: u32 = (*self).clone().into();

        // Increment the integer value
        let next_value = current_value + 1;

        // Create a new RomanNumber from the incremented value
        let next_roman = RomanNumber::from(next_value);

        // Update self to the new value
        *self = next_roman.clone();

        // Return the updated RomanNumber
        Some(next_roman)
    }
}
