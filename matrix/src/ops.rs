use crate::{Matrix, Scalar};
use std::ops::Add;

impl<T: Scalar<Item = T> + std::ops::Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0[0].len() != other.0[0].len() || self.0.len() != other.0.len() {
            return None;
        }

        let mut matrix = Matrix::new();
        for (j, row) in self.0.iter().enumerate() {
            if j > 0 {
                matrix.0.push(Vec::new());
            }
            for (i, v) in row.iter().enumerate() {
                matrix.0[j].push(v.clone() + other.0[j][i].clone());
            }
        }

        Some(matrix)
    }
}

use std::ops::Sub;

impl<T: Scalar<Item = T> + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        if self.0[0].len() != other.0[0].len() || self.0.len() != other.0.len() {
            return None;
        }

        let mut matrix = Matrix::new();
        for (j, row) in self.0.iter().enumerate() {
            if j > 0 {
                matrix.0.push(Vec::new());
            }
            for (i, v) in row.iter().enumerate() {
                matrix.0[j].push(v.clone() - other.0[j][i].clone());
            }
        }

        Some(matrix)
    }
}
