use lalgebra_scalar::Scalar;
mod mult;
mod ops;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
    }
    // It returns the zero matrix of the size given by the row and
    // column parameters
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut matrix = Matrix(Vec::new());
        for _ in 0..row {
            matrix.0.push(vec![T::zero(); col]);
        }
        matrix
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::new();
        for y in 0..n {
            if y > 0 {
                matrix.0.push(Vec::new());
            }
            for x in 0..n {
                if y == x {
                    matrix.0[y].push(T::one());
                } else {
                    matrix.0[y].push(T::zero());
                }
            }
        }
        matrix
    }
}