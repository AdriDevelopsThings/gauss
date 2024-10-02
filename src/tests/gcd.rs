use crate::gcd::gcd;

#[test]
fn test_gcd() {
    assert_eq!(gcd(2212, 16848), 4);
    assert_eq!(gcd(912, 726), 6);
    assert_eq!(gcd(183, 199), 1);
}
