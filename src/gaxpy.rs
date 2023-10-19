pub use crate::*;

pub fn gaxpy_ligne(a: Matrix, x: &Matrix, y: &mut Matrix) {
    assert_eq!(a.p, x.n);
    assert_eq!(a.n, y.n);
    assert_eq!(x.p, y.p);
    for i in 0..y.n {
        y.set_line(i, a.line(i) * x + y.line(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaxpy_ligne() {
        let a = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let x = Matrix::new_column(vec![5, 6]);
        let mut y = Matrix::new_column(vec![7, 8]);
        gaxpy_ligne(a, &x, &mut y);
        let expected_y = Matrix::new_column(vec![24, 47]);
        assert_eq!(y, expected_y);
    }
}
