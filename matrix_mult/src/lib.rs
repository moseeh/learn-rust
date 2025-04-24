use std::fmt::Debug;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() || self.0[0].is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n]).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Mul<Output = T> + std::iter::Sum + Default,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows_a = self.number_of_rows();
        let cols_a = self.number_of_cols();
        let rows_b = rhs.number_of_rows();
        let cols_b = rhs.number_of_cols();

        if cols_a != rows_b {
            return None;
        }

        let mut result = vec![vec![T::default(); cols_b]; rows_a];

        for i in 0..rows_a {
            for j in 0..cols_b {
                result[i][j] = (0..cols_a).map(|k| self.0[i][k] * rhs.0[k][j]).sum();
            }
        }

        Some(Matrix(result))
    }
}

