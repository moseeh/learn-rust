pub fn factorial(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
        _ => {
            let mut n = 1;
            let mut current = num;
            loop {
                if current == 1 {
                    break;
                }
                n = n * current;
                current = current - 1;
            }
            n
        }
    }
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
