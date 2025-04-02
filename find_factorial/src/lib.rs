pub fn factorial(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
    }
    let mut n = 1;
    loop {
        if num == 1 {
            break
        }
        n = n * num;
        num = num -1
    }
    n
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
