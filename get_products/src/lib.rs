pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n <= 1 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(n);

    for i in 0..n {
        // Calculate product of all elements except the one at index i
        let product = arr
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .fold(1, |acc, (_, &val)| acc * val);

        result.push(product);
    }

    result
}
