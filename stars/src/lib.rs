pub fn stars(n: u32) -> String {
    let count: usize = 2_u32.pow(n).try_into().unwrap();
    "*".repeat(count)
}