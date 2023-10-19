pub use crate::*;

// Finds x such as `Lx=b`
pub fn solves_lower_triangular(l: &Matrix, b: &Matrix) -> Matrix {
    assert!(l.is_lower_triangular(), "l isn't lower triangular");
    assert!(b.is_column(), "b isn't a column");
    assert_eq!(l.n, b.n, "l and b have incompatible sizes");

    let mut x = Matrix::new(l.n, 1);
    for i in 0..l.n {
        let mut s = 0.0;
        for j in 0..i {
            s += l[(i, j)] * x[(j, 0)];
        }
        x[(i, 0)] = (b[(i, 0)] - s) / l[(i, i)];
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solves_lower_triangular() {
        let l = Matrix::from(vec![vec![1, 0, 0], vec![2, 3, 0], vec![4, 5, 6]]);
        let b = Matrix::new_column(vec![7, 8, 9]);
        let x = solves_lower_triangular(&l, &b);
        let b2 = l * x;
        assert_eq!(b, b2);
    }
}
