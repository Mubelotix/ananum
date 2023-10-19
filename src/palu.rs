pub use crate::*;

/// Finds U such as `A=LU`
pub fn lu_fact(a: &Matrix) -> Matrix {
    assert_eq!(a.n, a.p, "a isn't square");
    let (n, p) = (a.n, a.p);

    let mut u = a.to_owned();
    for j in 0..p-1 {
        let mut tau_j = (1.0 / u[(j,j)]) * u.column(j);
        for j2 in 0..=j {
            tau_j[j2] = 0.0;
        }

        let mut e_j = Matrix::new_column(n);
        e_j[j] = 1.0;

        let m_j = Matrix::id(n) - tau_j * e_j.t();

        u = m_j * u;
    }

    u
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lu_fact() {
        let a = Matrix::from(vec![vec![-1, -1, -1], vec![-1, 0, -1], vec![1, 2, 2]]);
        let u = lu_fact(&a);
        println!("{u:?}")
    }
}
