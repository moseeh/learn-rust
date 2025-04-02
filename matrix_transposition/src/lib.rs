#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    
    Matrix((a, c), (b, d))
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
