pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut num: i32 = 0;
    for (_, v) in h {
        if v > num {
            num = v
        }
    }
    num
}