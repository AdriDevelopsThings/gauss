use crate::fraction::Fraction;

#[test]
fn test_fraction_mul() {
    let a = Fraction::by_n_d(162, 512);
    let b = Fraction::by_n_d(-762, 123);
    let result = Fraction::by_n_d(-123444, 62976);
    assert_eq!(a * b, result);
}

#[test]
fn test_fraction_div() {
    let a = Fraction::by_n_d(162, 512);
    let b = Fraction::by_n_d(-123, 762);
    let result = Fraction::by_n_d(-123444, 62976);
    assert_eq!(a / b, result);
}

#[test]
fn test_fraction_add_easy() {
    let a = Fraction::by_n_d(162, 512);
    let b = Fraction::by_n_d(-762, 512);
    let result = Fraction::by_n_d(-600, 512);
    assert_eq!(a + b, result);
}

#[test]
fn test_fraction_add_complex() {
    let a = Fraction::by_n_d(331, 163);
    let b = Fraction::by_n_d(612, 253);
    let result = Fraction::by_n_d(183499, 41239);
    assert_eq!(a + b, result);
}

#[test]
fn test_fraction_sub() {
    let a = Fraction::by_n_d(162, 621);
    let b = Fraction::by_n_d(172, 123);
    let result = Fraction::by_n_d(-3218, 2829);
    assert_eq!(a - b, result);
}
