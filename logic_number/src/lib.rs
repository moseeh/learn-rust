pub fn number_logic(num: u32) -> bool {
    if num == 0 {
        return false;
    }
    let mut digit_count = 0;
    let mut temp = num;
    while temp > 0 {
        digit_count += 1;
        temp /= 10;
    }

    let mut sum = 0;
    let mut num2 = num;

    while num2 > 0 {
        let digit = num2 % 10;
        sum += digit.pow(digit_count);
        num2 /= 10;
    }

    sum == num
}
