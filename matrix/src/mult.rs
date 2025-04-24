use crate::Matrix;
use crate::Scalar;
use std::ops::Mul;

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut column = Vec::new();
        for row in &self.0 {
            for (i, v) in row.iter().enumerate() {
                if i == n {
                    column.push(v.clone());
                }
            }
        }
        column
    }
}

impl<T: Scalar<Item = T> + std::iter::Sum<<T as std::ops::Mul>::Output>> Mul for Matrix<T> {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        // If the number of columns of self match don't match the number of
        // number_of_rows of self don't match return None
        let row_lenght = self.number_of_rows();
        let col_lenght = rhs.number_of_cols();
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }
        let mut result: Matrix<T> = Matrix::zero(row_lenght, col_lenght);
        for j in 0..result.number_of_rows() {
            for i in 0..result.number_of_cols() {
                result.0[j][i] = self
                    .row(j)
                    .iter()
                    .zip(rhs.col(i).iter())
                    .map(|(x, y)| x.clone() * y.clone())
                    .sum();
            }
        }
        Some(result)
    }
}

