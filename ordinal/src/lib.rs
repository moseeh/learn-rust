pub fn num_to_ordinal(x: u32) -> String {
    let last_two = x % 100;
    if last_two >= 11 && last_two <= 13 {
        return format!("{}th", x);
    }
    match x % 10 {
        1 => format!("{}st", x),
        2 => format!("{}nd", x),
        3 => format!("{}rd", x),
        _ => format!("{}th", x),
    }
}
