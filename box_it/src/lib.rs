pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    Box::new(
        s.split_whitespace()
            .map(|s| {
                if let Some(stripped) = s.strip_suffix('k') {
                    stripped.parse::<u32>().expect("invalid number") * 1000
                } else {
                    s.parse::<u32>().expect("Invalid number")
                }
            })
            .collect(),
    )
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
