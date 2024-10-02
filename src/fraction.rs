use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::gcd::gcd;

#[derive(Clone, Copy, Debug)]
pub struct Fraction {
    numerator: i32,
    denominator: u32,
}

impl Fraction {
    pub fn null() -> Self {
        Self {
            numerator: 0,
            denominator: 0,
        }
    }

    pub fn by_int(i: i32) -> Self {
        Self {
            numerator: i,
            denominator: 1,
        }
    }

    pub fn fn_by_n_d(n: i32, d: u32) -> Self {
        Self {
            numerator: n,
            denominator: d,
        }
    }

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

    fn same_denominator(&mut self, rhs: &mut Fraction) {
        let gcd = gcd(self.denominator, rhs.denominator);
        let scd = (self.denominator * rhs.denominator) / gcd;

        self.numerator *= (scd / self.denominator) as i32;
        rhs.numerator *= (scd / rhs.denominator) as i32;

        self.denominator = scd;
        rhs.denominator = scd;
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
    }
}

impl Mul<Fraction> for Fraction {
    type Output = Fraction;
    fn mul(self, rhs: Fraction) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}
impl Mul<i32> for Fraction {
    type Output = Fraction;
    fn mul(self, rhs: i32) -> Self::Output {
        Fraction {
            numerator: rhs * self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Div<Fraction> for Fraction {
    type Output = Fraction;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Fraction) -> Self::Output {
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

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
