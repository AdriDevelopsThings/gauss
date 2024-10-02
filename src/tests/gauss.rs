use crate::{fraction::Fraction, gauss::gauss, matrix::Matrix};

#[test]
fn test_easy_gauss() {
    let matrix = Matrix::new([
        [
            Fraction::by_int(2),
            Fraction::by_int(2),
            Fraction::by_int(1),
        ],
        [
            Fraction::by_int(4),
            Fraction::by_int(1),
            Fraction::by_int(3),
        ],
        [
            Fraction::by_int(1),
            Fraction::by_int(1),
            Fraction::by_int(2),
        ],
    ]);
    let result = Matrix::new([
        [Fraction::by_int(1)],
        [Fraction::by_int(0)],
        [Fraction::by_int(2)],
    ]);
    let gauss = gauss(matrix, result);
    assert_eq!(
        gauss,
        Matrix::new([
            [Fraction::by_int(-3)],
            [Fraction::by_int(6)],
            [Fraction::by_int(34)]
        ])
    );
}
