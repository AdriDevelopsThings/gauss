use crate::{fraction::Fraction, matrix::Matrix};

/// do gauss algorithm to the equation matrix * x = result
/// x will be returned (if x exists as a rational number)
pub fn gauss<const N: usize>(
    mut matrix: Matrix<N, N>,
    mut result: Matrix<N, 1>,
) -> Option<Matrix<N, 1>> {
    // eliminate each column except the last one
    for col in 0..N - 1 {
        let mut v = matrix.get(col, col).unwrap();
        if v.is_null() {
            // we can't use this row to eliminate the column, find another row
            for row in col + 1..N {
                // find any row under this one
                v = matrix.get(row, col).unwrap();
                if !v.is_null() {
                    // this row seems to work, swap the old and the new one
                    matrix.swap_row(col, row).unwrap();
                    result.swap_row(col, row).unwrap();
                    break;
                }
            }

            if v.is_null() {
                // a suitable row couldn't be found, the equation system seems to be not solvable
                return None;
            }
        }

        // now eliminate the column in each row under this one
        for row in col + 1..N {
            if matrix.get(row, col).unwrap().is_null() {
                // already eliminated
                continue;
            }
            // now we add factor * row `col` to this row `row`
            // the factor is such that (row, col) is zero
            let factor = -matrix.get(row, col).unwrap() / v;
            matrix.set(row, col, Fraction::null()).unwrap(); // we already know that
            for further_cols in col + 1..N {
                // now apply this fact to the next columns (the left columns are already eliminated)
                let old_value = matrix.get(row, further_cols).unwrap();
                let add_value = factor * matrix.get(col, further_cols).unwrap();
                matrix
                    .set(row, further_cols, old_value + add_value)
                    .unwrap();
            }

            // add factor * row `col` to this row `row` in result
            let old_result = result.get(row, 0).unwrap();
            let add_value = result.get(col, 0).unwrap();
            result.set(row, 0, old_result + factor * add_value).unwrap();
        }
    }

    // if any row is completely eliminated the equation system is not solveable
    if matrix
        .0
        .iter()
        .any(|row| row.iter().all(|col| col.is_null()))
    {
        return None;
    }

    // now calculate each variable
    let mut variables = [Fraction::null(); N];
    for i in 0..N {
        let index = N - i - 1;

        // first add the values from the already calculated variables
        // v = (right columns next to the diagonal element) * variables
        let mut v = Fraction::null();
        for (col, variable) in variables.into_iter().enumerate().skip(index) {
            v = v + (variable * matrix.get(index, col).unwrap());
        }

        // diagonal * x + v = result
        // x = (result - v) / diagonal

        let diagonal = matrix.get(index, index).unwrap();
        variables[index] = (result.get(index, 0).unwrap() - v) / diagonal;
    }

    Some(Matrix(variables.map(|c| [c])))
}
