#![doc = include_str!("../README.md")]

mod fraction;
mod gauss;
mod gcd;
mod matrix;

#[cfg(test)]
mod tests;

pub use fraction::Fraction;
pub use matrix::Matrix;
pub use gauss::gauss;