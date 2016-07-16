/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mut product = Vec::with_capacity(mat1.len());

    for row1 in mat1.iter() {
        assert!(row1.len() == mat2.len());
        let length = mat2[0].len();
        let mut row = Vec::with_capacity(length);

        for i in 0..length {
            let mut value = 0.;
            for (j, val1) in row1.iter().enumerate() {
                value += val1 * mat2[j][i];
            }
            row.push(value);
        }
        product.push(row);
    }

    product
}