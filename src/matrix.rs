pub use crate::*;

#[derive(Clone, PartialEq)]
pub struct Matrix {
    pub n: usize,
    pub p: usize,
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

    pub fn new_column<T: Copy + Into<f64>>(column: Vec<T>) -> Self {
        let mut result = Matrix::new(column.len(), 1);
        for i in 0..column.len() {
            result[(i, 0)] = column[i].into();
        }
        result
    }

    pub fn new_line<T: Copy + Into<f64>>(line: Vec<T>) -> Self {
        let mut result = Matrix::new(1, line.len());
        for j in 0..line.len() {
            result[(0, j)] = line[j].into();
        }
        result
    }

    pub fn transpose(&self) -> Matrix {
        let mut transposed = Matrix::new(self.p, self.n);
        for i in 0..self.n {
            for j in 0..self.p {
                transposed[(j, i)] = self[(i, j)];
            }
        }
        transposed
    }

    pub fn t(&self) -> Matrix {
        self.transpose()
    }

    pub fn line(&self, i: usize) -> Matrix {
        let mut line = Matrix::new(1, self.p);
        for j in 0..self.p {
            line[(0, j)] = self[(i, j)];
        }
        line
    }

    pub fn set_line(&mut self, i: usize, line: Matrix) {
        assert!(i < self.n, "Cannot set line over matrix bounds");
        assert_eq!(line.n, 1, "Cannot set line with a non-line matrix");
        assert_eq!(line.p, self.p, "Cannot set line with a matrix of different size");
        for j in 0..self.p {
            self[(i, j)] = line[(0, j)];
        }
    }

    pub fn column(&self, j: usize) -> Matrix {
        let mut column = Matrix::new(self.n, 1);
        for i in 0..self.n {
            column[(i, 0)] = self[(i, j)];
        }
        column
    }

    pub fn set_column(&mut self, j: usize, column: Matrix) {
        assert!(j < self.p, "Cannot set column over matrix bounds");
        assert_eq!(column.p, 1, "Cannot set column with a non-column matrix");
        assert_eq!(column.n, self.n, "Cannot set column with a matrix of different size");
        for i in 0..self.n {
            self[(i, j)] = column[(i, 0)];
        }
    }
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("Matrix {}x{}:\n", self.n, self.p);
        for i in 0..self.n {
            for j in 0..self.p {
                result.push_str(&format!("{}", self[(i, j)]));
                result.push(' ');
            }
            result.push('\n');
        }
        write!(f, "{}", result)
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
        assert!(i < self.n, "Index i out of bounds");
        assert!(j < self.p, "Index j out of bounds");
        &self.data[i * self.p + j]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    #[track_caller]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        assert!(i < self.n, "Index i out of bounds");
        assert!(j < self.p, "Index j out of bounds");
        &mut self.data[i * self.p + j]
    }
}

impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    #[track_caller]
    fn add(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.n, rhs.n, "Cannot add matrices of different sizes");
        assert_eq!(self.p, rhs.p, "Cannot add matrices of different sizes");
        let mut result = Matrix::new(self.n, self.p);
        for i in 0..self.n {
            for j in 0..self.p {
                result[(i, j)] = self[(i, j)] + rhs[(i, j)];
            }
        }
        result
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;

    #[track_caller]
    fn mul(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.p, rhs.n, "Cannot multiply matrices of incompatible sizes: {:?} and {:?}", self, rhs);
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

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    #[track_caller]
    fn mul(self, rhs: Matrix) -> Self::Output {
        self.mul(&rhs)
    }
}

impl std::ops::Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut result = Matrix::new(rhs.n, rhs.p);
        for i in 0..rhs.n {
            for j in 0..rhs.p {
                result[(i, j)] = self * rhs[(i, j)];
            }
        }
        result
    }
}

impl std::ops::Mul<Matrix> for usize {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let self2 = self as f64;
        self2.mul(rhs)
    }
}

impl std::ops::Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs.mul(self)
    }
}

impl std::ops::Mul<usize> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: usize) -> Self::Output {
        rhs.mul(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let matrix = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 1)], 2.0);
        assert_eq!(matrix[(1, 0)], 3.0);
        assert_eq!(matrix[(1, 1)], 4.0);

        let column = Matrix::new_column(vec![1, 2]);
        assert_eq!(column[(0, 0)], 1.0);
        assert_eq!(column[(1, 0)], 2.0);

        let line = Matrix::new_line(vec![1, 2]);
        assert_eq!(line[(0, 0)], 1.0);
        assert_eq!(line[(0, 1)], 2.0);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let m2 = Matrix::from(vec![vec![5, 6], vec![7, 8]]);
        let m3 = m1 + m2;
        let expected_m3 = Matrix::from(vec![vec![6, 8], vec![10, 12]]);
        assert_eq!(m3, expected_m3);
    }

    #[test]
    fn test_transpose() {
        let m1 = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let m2 = m1.transpose();
        let expected_m2 = Matrix::from(vec![vec![1, 3], vec![2, 4]]);
        assert_eq!(m2, expected_m2);
    }

    #[test]
    fn test_line_column() {
        let m1 = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let c1 = m1.column(0);
        let expected_c1 = Matrix::from(vec![vec![1], vec![3]]);
        assert_eq!(c1, expected_c1);
        let l2 = m1.line(1);
        let expected_l2 = Matrix::from(vec![vec![3, 4]]);
        assert_eq!(l2, expected_l2);
    }

    #[test]
    fn test_mul_constant() {
        let m1 = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
        let m2 = 2 * m1;
        let expected_m2 = Matrix::from(vec![vec![2, 4], vec![6, 8]]);
        assert_eq!(m2, expected_m2);
    }
}
