/// roman_numbers_iterator/src/lib.rs
// Re-export the RomanNumber type from the roman_numbers crate
pub use roman_numbers::RomanNumber;

/// Implement Iterator for RomanNumber so that each call to `next` advances the value by 1.
impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        // Convert the current RomanNumber into its integer value.
        // This requires that RomanNumber implements `Into<u32>` or `TryInto<u32>`.
        let current_value: u32 = (*self).clone().into();

        // Compute the next integer value
        let next_value = current_value + 1;

        // Create a new RomanNumber from the next integer
        let next_roman = RomanNumber::from(next_value);

        // Update self to point to the new value
        *self = next_roman.clone();

        // Always return Some for unbounded iteration
        Some(next_roman)
    }
}
