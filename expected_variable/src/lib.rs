use case::CaseExt;
use edit_distance::edit_distance;

// This function calculates the similarity between an original and an expected string using the edit distance
// and returns a percentage similarity if it meets certain conditions.
pub fn expected_variable(original: &str, expected: &str) -> Option<String> {
    // If the original string contains a space, return None (no comparison)
    if original.contains(" ") {
        return None;
    }
    
    // If the original string does not contain an underscore or any uppercase letters, return None
    if !original.contains('_') && !original.chars().any(|c| c.is_ascii_uppercase()) {
        None
    } else {
        // Calculate the edit distance between the lowercase versions of the original and expected strings
        let diff = edit_distance(&original.to_lowercase(), &expected.to_lowercase());
        
        // If the edit distance is greater than the length of the original string, return None
        if diff > original.len() {
            return None;
        }
        
        // Find the larger of the two string lengths
        let bigger = std::cmp::max(original.len(), expected.len());
        
        // Calculate the percentage similarity based on the edit distance
        let res = ((bigger - diff) * 100) as f64 / bigger as f64;
        
        // Round the result up
        let resu = res.ceil();
        
        // If the percentage similarity is less than 50%, return None
        if resu < 50.0 {
            return None;
        }
        
        // Otherwise, return the percentage similarity as a string with a "%" sign
        return Some(resu.to_string() + &"%".to_string());
    }
}