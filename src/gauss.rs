use crate::{fraction::Fraction, matrix::Matrix};

pub fn gauss<const N: usize>(mut matrix: Matrix<N, N>, mut result: Matrix<N, 1>) -> Matrix<N, 1> {
    for col in 0..N - 1 {
        let v = matrix.get(col, col).unwrap();
        for row in col + 1..N - 1 {
            let factor = matrix.get(row, col).unwrap() / v;
            let add_value = factor * v;
            matrix.set(row, col, Fraction::null()).unwrap();
            for further_cols in col + 1..N - 1 {
                let old_value = matrix.get(row, further_cols).unwrap();
                matrix
                    .set(row, further_cols, old_value + add_value)
                    .unwrap();
            }

            let old_result = result.get(row, 0).unwrap();
            result.set(row, 0, old_result + add_value).unwrap();
        }
    }

    let mut variables = [Fraction::null(); N];
    for i in 0..N - 1 {
        let index = N - i - 1;

        let mut v = Fraction::null();
        for col in index..N - 1 {
            v = v + (variables[col] * matrix.get(index, col).unwrap());
        }

        let diagonal = matrix.get(index, index).unwrap();
        variables[index] = (result.get(index, 0).unwrap() - v) / diagonal;
    }

    Matrix::new(variables.map(|c| [c]))
}
