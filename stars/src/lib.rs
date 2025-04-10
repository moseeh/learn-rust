pub fn stars(n: u32) -> String {
    "*".repeat(n.try_into().unwrap())
}
