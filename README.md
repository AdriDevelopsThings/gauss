# gauss
Solve equation systems by using the gauss algorithm.


## Example
```rust
use gauss::{Fraction, Matrix, gauss};

fn main() {
    // 2x + 3y = 9
    // 9x - 2y = 12

    let matrix = Matrix::by_integers([
        [2, 3],
        [9, -2]
    ]);
    let result = Matrix::by_integers([
        [9],
        [12]
    ]);

    let x_y = gauss(matrix, result);
    assert_eq!(x_y, Some(Matrix([
        [Fraction::by_n_d(54, 31)],
        [Fraction::by_n_d(57, 31)]
    ])));
    // x = 54/31
    // y = 57/31

    let x_y = x_y.unwrap();

    let x: f32 = x_y.get(0, 0).unwrap().into();
    let y: f32 = x_y.get(1, 0).unwrap().into();
    println!("Result is x={x}, y={y}");
}
```