use std::fmt::Display;

use crate::fraction::Fraction;

#[derive(PartialEq, Debug)]
pub struct Matrix<const R: usize, const C: usize>([[Fraction; C]; R]);

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(matrix: [[Fraction; C]; R]) -> Self {
        Self(matrix)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Fraction> {
        if row >= R || col >= C {
            None
        } else {
            Some(self.0[row][col])
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: Fraction) -> Result<(), &'static str> {
        if row >= R || col >= C {
            Err("row or col value for Matrix::set is not in the right range")
        } else {
            self.0[row][col] = value;
            Ok(())
        }
    }
}

impl<const R: usize, const C: usize> Display for Matrix<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..R - 1 {
            for col in 0..C - 1 {
                write!(f, "{} ", self.0[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
