pub use crate::*;

/// Finds U such as `A=LU`
pub fn find_u(a: &Matrix) -> Matrix {
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

// broken
pub fn lu_fact(a: &Matrix) -> (Matrix, Matrix) {
    assert_eq!(a.n, a.p, "a isn't square");
    assert!((0..a.n).map(|k| a[(k,k)]).all(|v| v!=0.0), "zeroes in diagonal");
    let (_n, p) = (a.n, a.p);

    let mut u = a.to_owned();
    let mut l = Matrix::id(a.n);
    for k in 0..p-1 {
        update_l(&mut l, &u, k);
        println!("l {l:?}");
        //u = m_j * u;
        tf_gauss(&mut u, k);

    }

    (l, u)
}

// broken
fn tf_gauss(a: &mut Matrix, k: usize) {
    for i in k+1..a.n {
        for j in k..a.n {
            a[(i, j)] -= (a[(i, k)]/a[(k, k)]) * a[(k, j)];
        }
    }
}

// broken
fn update_l(l: &mut Matrix, u: &Matrix, k: usize) {
    for i in k+1..u.n {
        l[(i, k)] = u[(i, k)]/u[(k,k)];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lu_fact() {
        let a = Matrix::from(vec![vec![-1, -1, -1], vec![-1, 1, -1], vec![1, 2, 2]]);
        let u = find_u(&a);
        println!("{u:?}")
    }
}
