use std::fmt::Display;

use crate::fraction::Fraction;

/// this struct should represent a R x C matrix
#[derive(PartialEq, Debug)]
pub struct Matrix<const R: usize, const C: usize>(pub [[Fraction; C]; R]);

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn by_integers(matrix: [[i32; R]; C]) -> Matrix<C, R> {
        Matrix(matrix.map(|row| row.map(Fraction::by_int)))
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

    pub fn swap_row(&mut self, row1: usize, row2: usize) -> Result<(), &'static str> {
        if row1 >= C || row2 >= C {
            Err("row1 or row2 value for Matrix::swap_row is not in the right range")
        } else {
            self.0.swap(row1, row2);
            Ok(())
        }
    }
}

/// a matrix will be displayed like this
/// "a11 a12
/// a21 a22"
impl<const R: usize, const C: usize> Display for Matrix<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..R {
            for col in 0..C {
                write!(f, "{} ", self.0[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
