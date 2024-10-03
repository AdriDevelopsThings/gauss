use crate::{fraction::Fraction, gauss::gauss, matrix::Matrix};

fn str_to_fraction(s: &str) -> Fraction {
    if s.contains('/') {
        let a = s.split('/').collect::<Vec<&str>>();
        assert_eq!(a.len(), 2);
        Fraction::by_n_d(a[0].parse().unwrap(), a[1].parse().unwrap())
    } else {
        Fraction::by_int(s.parse().unwrap())
    }
}

/// matrix from format:
/// a b c d
/// a b c d
/// a b c d
fn matrices_from_str(s: &'static str) -> (Matrix<3, 3>, Matrix<3, 1>) {
    let mut matrix = [[Fraction::null(); 3]; 3];
    let mut result = [[Fraction::null(); 1]; 3];

    for (row_index, row) in s.split('\n').enumerate() {
        let cols = row.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(cols.len(), 4);
        let values = cols.iter().map(|n| str_to_fraction(n));
        matrix[row_index] = (*values.clone().take(3).collect::<Vec<Fraction>>().as_slice())
            .try_into()
            .unwrap();
        result[row_index][0] = values.last().unwrap();
    }

    (Matrix(matrix), Matrix(result))
}

fn vector_from_str(s: &'static str) -> Matrix<3, 1> {
    let l: [[Fraction; 1]; 3] = s
        .split_whitespace()
        .map(|r| [str_to_fraction(r)])
        .collect::<Vec<[Fraction; 1]>>()
        .try_into()
        .unwrap();
    Matrix(l)
}

fn gauss_by_str(s: &'static str) -> Option<Matrix<3, 1>> {
    let (matrix, result) = matrices_from_str(s);
    gauss(matrix, result)
}

#[test]
fn test_gauss() {
    assert_eq!(
        gauss_by_str(
            "2 2 1 1
            4 1 3 0
            1 1 2 2"
        ),
        Some(vector_from_str("-1 1 1"))
    );

    assert_eq!(
        gauss_by_str(
            "-3 4 -3 -5
            3 -2 3 7
            -2 4 -2 -1"
        ),
        None
    );

    assert_eq!(
        gauss_by_str(
            "1 2 3 9
            2 3 1 8
            3 1 2 7"
        ),
        Some(vector_from_str("2/3 5/3 5/3"))
    );

    assert_eq!(
        gauss_by_str(
            "5 -3 1 8
            2 1 -4 -3
            3 2 5 12"
        ),
        Some(vector_from_str("185/132 1/6 197/132"))
    );
}

#[test]
fn test_gauss_4() {
    let matrix = Matrix::by_integers(
        [[2, 1, 4, 3], [-1, 2, 1, -1], [3, 4, -1, -2], [4, 3, 2, 1]],
    );

    let result = Matrix::by_integers([[0], [4], [0], [0]]);

    let solution = Matrix::by_integers([[2], [-4], [6], [-8]]);

    assert_eq!(gauss(matrix, result), Some(solution));
}
