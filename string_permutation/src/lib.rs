pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_counts = std::collections::HashMap::new();
    for c in s1.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }
    char_counts.values().all(|&count| count == 0)
}