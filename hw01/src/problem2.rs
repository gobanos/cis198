/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mut product = Vec::with_capacity(mat1.len());

    for (i, row1) in mat1.iter().enumerate() {
        assert!(row1.len() = mat2.len());
        let mut row = Vec::with_capacity(mat2[0].len());

        // TODO
    }

    product
}