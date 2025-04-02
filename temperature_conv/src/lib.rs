pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32) * (5 / 9)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9 / 5) + 32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
