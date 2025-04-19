use std::ops::{Add, Mul};


/// A mathematical vector of scalars.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        let data = self
            .0
            .into_iter()
            .zip(other.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Some(Vector(data))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum = T::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            sum = sum + (*a * *b);
        }
        Some(sum)
    }
}
