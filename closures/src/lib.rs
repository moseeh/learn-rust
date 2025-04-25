pub fn first_fifty_even_square() -> Vec<i32> {
    (1..51).map(|n| (n * 2) as i32).map(|n| n * n).collect()
}
