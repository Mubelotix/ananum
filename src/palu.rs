pub use crate::*;

/// Finds U such as `A=LU`
pub fn lu_fact(a: &Matrix) -> Matrix {
    assert_eq!(a.n, a.p, "a isn't square");
    let (n, p) = (a.n, a.p);

    let mut u = a.to_owned();
    for k in 0..p-1 {
        let mut tau_k = (1.0 / u[(k,k)]) * u.column(k);
        for j2 in 0..=k {
            tau_k[j2] = 0.0;
        }

        let mut e_k = Matrix::new_column(n);
        e_k[k] = 1.0;

        let m_j = Matrix::id(n) - tau_k * e_k.t();

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
