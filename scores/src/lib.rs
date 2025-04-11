pub fn score(s: &str) -> u64 {
    let mut total : u64 = 0;
    for c in s.chars() {
        let letter = c.to_ascii_uppercase();
        if !letter.is_ascii_alphabetic() {
            continue
        }

        let value = match letter {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        };
        total += value;
    }
    total

}