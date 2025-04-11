pub fn talking(text: &str) -> &str {
    if text == "" {
        return "Just say something!"
    }
    if text.trim_end().ends_with('?') {
        if is_all_uppercase(text) {
            return "Quiet, I am thinking!";
        }
        return "Sure."
    } else {
        if is_all_uppercase(text) {
            return "There is no need to yell, calm down!"
        }
    }
    "Interesting"
}
pub fn is_all_uppercase(s: &str) -> bool {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| c.is_ascii_uppercase())
}
