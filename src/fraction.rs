use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::gcd::gcd;

/// This struct demonstrates a rational number
/// The number has the format of a fraction:
/// number = numerator / denominator
///
/// All states are valid except that the denominator is 0.
/// A zero is represented as numerator=0 and denominator=don't care, could be any value
#[derive(Clone, Copy, Debug)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: u32,
}

impl Fraction {
    pub fn null() -> Self {
        Self::by_int(0)
    }

    /// construct the fraction by an integer `i`
    pub fn by_int(i: i32) -> Self {
        Self {
            numerator: i,
            denominator: 1,
        }
    }

    /// construct the fraction by its numerator `n` and denominator `d`
    pub fn by_n_d(n: i32, d: u32) -> Self {
        Self {
            numerator: n,
            denominator: d,
        }
    }

    pub fn is_null(&self) -> bool {
        self.numerator == 0
    }

    /// get the reciprocal value (inverse the fraction)
    fn inverse(&self) -> Self {
        let negative = self.numerator.is_negative();
        Self {
            numerator: (self.denominator as i32)
                * match negative {
                    true => -1,
                    false => 1,
                },
            denominator: self.numerator.unsigned_abs(),
        }
    }

    /// bring both fractions to the same denominator
    fn same_denominator(&mut self, rhs: &mut Fraction) {
        let gcd = gcd(self.denominator, rhs.denominator);
        let scd = (self.denominator * rhs.denominator) / gcd;

        self.numerator *= (scd / self.denominator) as i32;
        rhs.numerator *= (scd / rhs.denominator) as i32;

        self.denominator = scd;
        rhs.denominator = scd;
    }

    /// simplify a fraction by finding the gcd of both denominators
    fn to_simplified(self) -> Self {
        let gcd = gcd(self.numerator.unsigned_abs(), self.denominator);
        if gcd != 1 {
            Self {
                numerator: self.numerator / gcd as i32,
                denominator: self.denominator / gcd,
            }
        } else {
            self
        }
    }
}

impl Neg for Fraction {
    type Output = Fraction;
    fn neg(self) -> Self::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Add<Fraction> for Fraction {
    type Output = Fraction;
    fn add(mut self, mut rhs: Fraction) -> Self::Output {
        if self.numerator == 0 {
            return rhs;
        }
        if rhs.numerator == 0 {
            return self;
        }

        if self.denominator != rhs.denominator {
            self.same_denominator(&mut rhs);
        }

        Self {
            numerator: self.numerator + rhs.numerator,
            denominator: self.denominator,
        }
        .to_simplified()
    }
}

impl Mul<Fraction> for Fraction {
    type Output = Fraction;
    fn mul(self, rhs: Fraction) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
        .to_simplified()
    }
}
impl Mul<i32> for Fraction {
    type Output = Fraction;
    fn mul(self, rhs: i32) -> Self::Output {
        Fraction {
            numerator: rhs * self.numerator,
            denominator: self.denominator,
        }
        .to_simplified()
    }
}

impl Div<Fraction> for Fraction {
    type Output = Fraction;
    /// dividing a fraction by zero causes a divide by zero panic
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Fraction) -> Self::Output {
        if rhs.is_null() {
            panic!("attempt to divide by zero");
        }
        // dividing a fraction by another is the same as multiplying it by the inverse of the other fraction
        self * rhs.inverse()
    }
}

impl Sub<Fraction> for Fraction {
    type Output = Fraction;
    fn sub(self, rhs: Fraction) -> Self::Output {
        self + (rhs * -1)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        if self.numerator == other.numerator && self.denominator == other.denominator {
            return true;
        }

        if self.numerator == 0 && other.numerator == 0 {
            return true;
        }

        if self.numerator == 0 || other.numerator == 0 {
            return false;
        }

        let mut one = *self;
        let mut two = *other;
        one.same_denominator(&mut two);
        one.numerator == two.numerator
    }
}

impl From<Fraction> for f32 {
    fn from(value: Fraction) -> Self {
        value.numerator as f32 / value.denominator as f32
    }
}

impl From<Fraction> for f64 {
    fn from(value: Fraction) -> Self {
        value.numerator as f64 / value.denominator as f64
    }
}

/// Fractions are displayed like "{numerator}/{denominator}" except that
/// the fraction is an integer then it will be displayed like "{numerator/denominator}"
impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.numerator % self.denominator as i32 == 0 {
            write!(f, "{}", self.numerator / self.denominator as i32)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}
