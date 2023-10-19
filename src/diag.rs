pub use crate::*;

// Finds x such as `Dx=b`
pub fn solves_diag(d: &Matrix, b: &Matrix) -> Matrix {
    assert!(d.is_diag(), "d isn't diagonal");
    assert!(b.is_column(), "b isn't a column");
    assert_eq!(d.n, b.n, "incompatible sizes");
    let mut x = Matrix::new(d.p, 1);
    for k in 0..d.n {
        x[(k, 0)] = b[(k, 0)] / d[(k, k)];
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solves_diag() {
        let d = Matrix::new_diag(vec![1, 2, 3, 4]);
        let b = Matrix::new_column(vec![5, 6, 7, 8]);
        let x = solves_diag(&d, &b);
        let b2 = d * x;
        assert_eq!(b, b2);
    }
}
