pub fn bubble_sort(slice: &mut [i32]) {
    let n = slice.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}