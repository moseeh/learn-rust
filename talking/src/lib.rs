pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();
    if trimmed == "" {
        return "Just say something!";
    }
    if trimmed.ends_with('?') {
        if is_all_uppercase(trimmed) {
            return "Quiet, I am thinking!";
        }
        return "Sure.";
    } else {
        if is_all_uppercase(trimmed) {
            return "There is no need to yell, calm down!";
        }
    }
    "Interesting"
}
pub fn is_all_uppercase(s: &str) -> bool {
    let letters: Vec<char> = s.chars().filter(|c| c.is_ascii_alphabetic()).collect();

    !letters.is_empty() && letters.iter().all(|c| c.is_ascii_uppercase())
}
