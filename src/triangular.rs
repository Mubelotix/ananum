pub use crate::*;

/// Finds x such as `Lx=b`
pub fn solves_lower_triangular(l: &Matrix, b: &Matrix) -> Matrix {
    assert!(l.is_lower_triangular(), "l isn't lower triangular");
    assert!(b.is_column(), "b isn't a column");
    assert_eq!(l.n, b.n, "l and b have incompatible sizes");

    let mut x = Matrix::new(l.n, 1);
    for i in 0..l.n {
        let mut s = 0.0;
        for j in 0..i {
            s += l[(i, j)] * x[j];
        }
        x[i] = (b[i] - s) / l[(i, i)];
    }
    x
}

/// Finds x such as `Lx=b`
/// Trough intermediaries like `M3 M2 M1 Lx = M3 M2 M1 b`
pub fn solves_lower_triangular_mat(l: &Matrix, b: &Matrix) -> Matrix {
    assert!(l.is_lower_triangular(), "l isn't lower triangular");
    assert!(b.is_column(), "b isn't a column");
    assert_eq!(l.n, b.n, "l and b have incompatible sizes");

    let mut x = b.to_owned();
    for j in 0..l.p {
        let mut tau_j = (1.0 / l[(j,j)]) * l.column(j);
        tau_j[j] = 1.0 - (1.0 / l[(j,j)]);
        for j2 in 0..j {
            tau_j[j2] = 0.0;
        }

        let mut e_j = Matrix::new_column(l.n);
        e_j[j] = 1.0;

        let m_j = Matrix::id(l.n) - tau_j * e_j.t();

        x = m_j * x;
    }

    x
}

pub fn invert(l: &Matrix) -> Matrix {
    let b = Matrix::from_column(vec![1; l.n]);

    assert!(l.is_lower_triangular(), "l isn't lower triangular");
    assert!(b.is_column(), "b isn't a column");
    assert_eq!(l.n, b.n, "l and b have incompatible sizes");

    let mut ms = Vec::new();
    for j in 0..l.p {
        let mut tau_j = (1.0 / l[(j,j)]) * l.column(j);
        tau_j[j] = 1.0 - (1.0 / l[(j,j)]);
        for j2 in 0..j {
            tau_j[j2] = 0.0;
        }

        let mut e_j = Matrix::new_column(l.n);
        e_j[j] = 1.0;

        let m_j = Matrix::id(l.n) - tau_j * e_j.t();

        ms.push(m_j);
    }

    let mut inv = Matrix::id(l.n);
    for m in ms.iter().rev() {
        inv = inv * m;
    }
    inv
}

pub fn solves_upper_triangular(u: &Matrix, b: &Matrix) -> Matrix {
    assert!(u.is_upper_triangular(), "u isn't upper triangular");
    assert!(b.is_column(), "b isn't a column");
    assert_eq!(u.n, b.n, "u and b have incompatible sizes");
    let (n, _p) = (u.n, u.p);

    let mut x = Matrix::new_column(n);
    x[n-1] = b[n-1] / u[(n-1,n-1)];

    for i in (0..n-1).rev() {
        let mut s = 0.0;
        for j in i+1..n {
            s += u[(i,j)] * x[j];
        }
        x[i] = (b[i] - s) / u[(i,i)];
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solves_lower_triangular() {
        let l = Matrix::from(vec![vec![1, 0, 0], vec![2, 3, 0], vec![4, 5, 6]]);
        let b = Matrix::from_column(vec![7, 8, 9]);
        let x = solves_lower_triangular(&l, &b);
        let b2 = l * x;
        assert!(b.all_close(&b2));
    }

    #[test]
    fn test_solves_lower_triangular_mat() {
        let l = Matrix::from(vec![vec![1, 0, 0], vec![2, 3, 0], vec![4, 5, 6]]);
        let b = Matrix::from_column(vec![7, 8, 9]);
        let x = solves_lower_triangular_mat(&l, &b);
        let b2 = l * x;
        assert!(b.all_close(&b2));
    }

    #[test]
    fn test_solves_upper_triangular() {
        let u = Matrix::from(vec![vec![1, 2], vec![0, 3]]);
        let b = Matrix::from_column(vec![4, 5]);
        let x = solves_upper_triangular(&u, &b);
        let b2 = u * x;
        println!("{b2:?}");
        assert!(b.all_close(&b2));
    }

    #[test]
    fn test_inv() {
        let l = Matrix::from(vec![vec![1, 0, 0], vec![2, 3, 0], vec![4, 5, 6]]);
        let inv = invert(&l);
        let prod = l * &inv;
        let id = Matrix::id(3);
        assert!(prod.all_close(&id));
    }
}
