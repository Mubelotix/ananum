pub use crate::*;

pub struct Matrix {
    n: usize,
    p: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(n: usize, p: usize) -> Self {
        Self {
            n,
            p,
            data: vec![0.0; n * p],
        }
    }
}

impl<T: Copy + Into<f64>> From<Vec<Vec<T>>> for Matrix {
    fn from(value: Vec<Vec<T>>) -> Self {
        let n = value.len();
        let p = value[0].len();
        assert!(value.iter().all(|row| row.len() == p));
        let mut result = Self::new(n, p);
        for i in 0..n {
            for j in 0..p {
                result[(i, j)] = value[i][j].into();
            }
        }
        result
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    #[track_caller]
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        assert!(i < self.n);
        assert!(j < self.p);
        &self.data[i * self.p + j]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    #[track_caller]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        assert!(i < self.n);
        assert!(j < self.p);
        &mut self.data[i * self.p + j]
    }
}

impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    #[track_caller]
    fn add(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.n, rhs.n);
        assert_eq!(self.p, rhs.p);
        let mut result = Matrix::new(self.n, self.p);
        for i in 0..self.n {
            for j in 0..self.p {
                result[(i, j)] = self[(i, j)] + rhs[(i, j)];
            }
        }
        result
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    #[track_caller]
    fn mul(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.p, rhs.n);
        let mut result = Matrix::new(self.n, rhs.p);
        for i in 0..self.n {
            for j in 0..rhs.p {
                for k in 0..self.p {
                    result[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        result
    }
}
