#![allow(dead_code)]

use crate::tuple::Tuple;
use crate::utils::FuzzyEq;
use crate::F;
use std::fmt;
use std::ops::{Index, IndexMut, Mul};

#[derive(Copy, Clone)]
struct Matrix<const D: usize>([[F; D]; D]);

#[macro_export]
macro_rules! matrix {
    () => {
        Matrix::new()
    };

    ($a:expr, $b:expr;
     $c:expr, $d:expr$(;)?) => {
        Matrix([[$a as F, $b as F], [$c as F, $d as F]])
    };

    ($a:expr, $b:expr, $c:expr;
     $d:expr, $e:expr, $f:expr;
     $g:expr, $h:expr, $i:expr$(;)?) => {
        Matrix([
            [$a as F, $b as F, $c as F],
            [$d as F, $e as F, $f as F],
            [$g as F, $h as F, $i as F],
        ])
    };

    ($a:expr, $b:expr, $c:expr, $d:expr;
     $e:expr, $f:expr, $g:expr, $h:expr;
     $i:expr, $j:expr, $k:expr, $l:expr;
     $m:expr, $n:expr, $o:expr, $p:expr$(;)?) => {
        Matrix([
            [$a as F, $b as F, $c as F, $d as F],
            [$e as F, $f as F, $g as F, $h as F],
            [$i as F, $j as F, $k as F, $l as F],
            [$m as F, $n as F, $o as F, $p as F],
        ])
    };
}

impl<const D: usize> Matrix<D> {
    pub fn new() -> Self {
        Self([[0.0; D]; D])
    }

    pub fn size(&self) -> usize {
        D
    }

    pub fn transpose(&self) -> Self {
        let mut matrix = matrix![];

        for row in 0..self.size() {
            for col in 0..self.size() {
                matrix[col][row] = self[row][col];
            }
        }

        matrix
    }
}

impl Matrix<2> {
    pub fn determinant(&self) -> F {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() == 0.0
    }
}

impl Matrix<3> {
    pub fn determinant(&self) -> F {
        let mut det = 0.0;

        for c in 0..self.size() {
            det += self[0][c] * self.cofactor(0, c);
        }

        det
    }

    pub fn minor(&self, row: usize, col: usize) -> F {
        self.submatrix(row, col).determinant()
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let mut matrix = matrix![];

        for r in 0..self.size() - 1 {
            for c in 0..self.size() - 1 {
                let row = if r >= row { r + 1 } else { r };
                let col = if c >= col { c + 1 } else { c };

                matrix[r][c] = self[row][col];
            }
        }

        matrix
    }

    pub fn cofactor(&self, row: usize, col: usize) -> F {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn is_invertible(&self) -> bool {
        self.determinant() == 0.0
    }
}

impl Matrix<4> {
    pub fn identity() -> Self {
        matrix![
            1, 0, 0, 0;
            0, 1, 0, 0;
            0, 0, 1, 0;
            0, 0, 0, 1;
        ]
    }

    pub fn determinant(&self) -> F {
        let mut det = 0.0;

        for c in 0..self.size() {
            det += self[0][c] * self.cofactor(0, c);
        }

        det
    }

    pub fn cofactor(&self, row: usize, col: usize) -> F {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> F {
        self.submatrix(row, col).determinant()
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let mut matrix = matrix![];

        for r in 0..self.size() - 1 {
            for c in 0..self.size() - 1 {
                let row = if r >= row { r + 1 } else { r };
                let col = if c >= col { c + 1 } else { c };

                matrix[r][c] = self[row][col];
            }
        }

        matrix
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Self {
        assert!(self.is_invertible());

        let det = self.determinant();

        let mut matrix = matrix![];

        for row in 0..self.size() {
            for col in 0..self.size() {
                matrix[col][row] = self.cofactor(row, col) / det;
            }
        }

        matrix
    }
}

impl<const D: usize> Index<usize> for Matrix<D> {
    type Output = [F; D];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const D: usize> IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const D: usize> Mul<Matrix<D>> for Matrix<D> {
    type Output = Matrix<D>;

    fn mul(self, other: Matrix<D>) -> Self::Output {
        let mut matrix = matrix![];

        for row in 0..D {
            for col in 0..D {
                for i in 0..D {
                    matrix[row][col] += self[row][i] * other[i][col];
                }
            }
        }

        matrix
    }
}

impl<const D: usize> Mul<Tuple> for Matrix<D> {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        let mut tuple = Tuple::from((0, 0, 0, 0));

        for row in 0..D {
            for col in 0..D {
                tuple[row] += self[row][col] * other[col];
            }
        }

        tuple
    }
}

impl<const D: usize> FuzzyEq<Self> for Matrix<D> {
    fn fuzzy_eq(&self, other: Self) -> bool {
        for row in 0..D {
            for column in 0..D {
                if !self[row][column].fuzzy_eq(other[row][column]) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const D: usize> fmt::Display for Matrix<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;

        for row in 0..self.size() {
            write!(f, "[")?;

            for col in 0..self.size() {
                write!(f, "{:>8.3}", self[row][col])?;
            }

            writeln!(f, "  ]")?;
        }

        Ok(())
    }
}

impl<const D: usize> fmt::Debug for Matrix<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix2() {
        let m = matrix![
          -3, 5;
           1, 2
        ];

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], 2.0);
    }

    #[test]
    fn matrix3() {
        let m = matrix![
         -3 ,  5 ,  0;
          1 , -2 , -7;
          0,   1,   1;
        ];

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix4() {
        let m = matrix![
            1,       2,    3,    4;
            5.5,   6.5,  7.5,  8.5;
            9,      10,   11,   12;
            13.5, 14.5, 15.5, 16.5
        ];

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    #[test]
    fn identical_matrices() {
        let a = matrix![
          1 ,2 ,3 ,4;
          5 ,6 ,7 ,8;
          9 ,8 ,7 ,6;
          5 ,4 ,3 ,2;
        ];

        let b = matrix![
          1 ,2 ,3 ,4;
          5 ,6 ,7 ,8;
          9 ,8 ,7 ,6;
          5 ,4 ,3 ,2;
        ];

        assert_fuzzy_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = matrix![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2;
        ];
        let b = matrix![
            2, 3, 4, 5;
            6, 7, 8, 9;
            8, 7, 6, 5;
            4, 3, 2, 1;
        ];

        assert_fuzzy_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = matrix![
          1,  2, 3,  4;
          5,  6, 7,  8;
          9,  8, 7,  6;
          5,  4, 3,  2;
        ];

        let b = matrix![
         -2,  1, 2,  3;
          3,  2, 1, -1;
          4,  3, 6,  5;
          1,  2, 7,  8;
        ];

        let expected = matrix![
           20, 22 , 50 , 48;
           44, 54 , 114 , 108;
           40, 58 , 110 , 102;
           16, 26 , 46 , 42;
        ];

        assert_fuzzy_eq!(a * b, expected);
    }

    #[test]
    fn multiplying_matrix_by_tuple() {
        let a = matrix![
          1,2,3,4;
          2,4,4,2;
          8,6,4,1;
          0,0,0,1;
        ];

        let b = Tuple::from((1, 2, 3, 1));

        let expected = Tuple::from((18, 24, 33, 1));

        assert_fuzzy_eq!(a * b, expected);
    }

    #[test]
    fn multiplying_matrix_by_identity_matrix() {
        let a = matrix![
            0,  1,  2,  4;
            1,  2,  4,  8;
            2,  4,  8, 16;
            4,  8, 16, 32;
        ];

        assert_fuzzy_eq!(a * Matrix::identity(), a);
    }

    #[test]
    fn multiplying_identity_matrix_by_tuple() {
        let tuple = Tuple::from((1, 2, 3, 4));

        assert_fuzzy_eq!(Matrix::identity() * tuple, tuple);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = matrix![
            0,9,3,0;
            9,8,0,8;
            1,8,5,3;
            0,0,5,8;
        ];

        let transposed = matrix![
            0,9,1,0;
            9,8,8,0;
            3,0,5,5;
            0,8,3,8;
        ];

        assert_fuzzy_eq!(a.transpose(), transposed);
    }

    #[test]
    fn transposing_identity_matrix() {
        let matrix = Matrix::identity();

        assert_fuzzy_eq!(matrix.transpose(), matrix);
    }

    #[test]
    fn determinant_of_a2x2_matrix() {
        let a = matrix![
             1, 5;
            -3, 2;
        ];

        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_a3x3_matrix_is_a_2x2_matrix() {
        let a = matrix![
            1, 5,  0;
           -3, 2,  7;
            0, 6, -3;
        ];

        let b = matrix![
            -3, 2;
             0, 6;
        ];

        assert_fuzzy_eq!(a.submatrix(0, 2), b);
    }

    #[test]
    fn submatrix_of_a4x4_matrix_is_a_3x3_matrix() {
        let a = matrix![
            -6 , 1 , 1 , 6;
            -8 , 5 , 8 , 6;
            -1 , 0 , 8 , 2;
            -7 , 1 , -1 , 1;
        ];

        let b = matrix![
            -6 , 1 , 6;
            -8 , 8 , 6;
            -7 , -1 , 1;
        ];

        assert_fuzzy_eq!(a.submatrix(2, 1), b);
    }

    #[test]
    fn minor_of_a_3x3_matrix() {
        let a = matrix![
            3 ,  5 ,  0;
            2 , -1 , -7;
            6 , -1 ,  5;
        ];

        assert_eq!(a.minor(1, 0), 25.0)
    }

    #[test]
    fn cofactor_of_a_3x3_matrix() {
        let a = matrix![
            3,  5,  0;
            2, -1, -7;
            6, -1,  5;
        ];

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0)
    }

    #[test]
    fn determinant_of_a_3x3_matrix() {
        let a = matrix![
            1,  2,  6;
           -5,  8, -4;
            2,  6,  4;
        ];

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn calculating_determinant_of_a_4x4_matrix() {
        let a = matrix![
           -2, -8,  3,  5;
           -3,  1,  7,  3;
            1,  2, -9,  6;
           -6,  7,  7, -9;
        ];

        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn matrix_is_invertible() {
        let a = matrix![
            6,  4,  4,  4;
            5,  5,  7,  6;
            4, -9,  3, -7;
            9,  1,  7, -6;
        ];

        assert_eq!(a.determinant(), -2120.0);

        assert!(a.is_invertible());
    }

    #[test]
    fn matrix_is_not_invertible() {
        let a = matrix![
            -4 ,  2 , -2 , -3;
             9 ,  6 ,  2 ,  6;
             0 , -5 ,  1 , -5;
             0 ,  0 ,  0 ,  0;
        ];

        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn inverse_of_a_matrix() {
        let a = matrix![
            -5,  2,  6, -8;
             1, -5,  1,  8;
             7,  7, -6, -7;
             1, -3,  7,  4;
        ];

        let b = a.inverse();

        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[3][2], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[2][3], 105.0 / 532.0);

        assert_fuzzy_eq!(
            b,
            matrix![
               0.21805,  0.45113,  0.24060, -0.04511;
              -0.80827, -1.45677, -0.44361,  0.52068;
              -0.07895, -0.22368, -0.05263,  0.19737;
              -0.52256, -0.81391, -0.30075,  0.30639;
            ]
        );
    }

    #[test]
    fn inverse_of_another_matrix() {
        let a = matrix![
            8, -5,  9,  2;
            7,  5,  6,  1;
           -6,  0,  9,  6;
           -3,  0, -9, -4;
        ];

        let inverse = matrix![
           -0.15385, -0.15385, -0.28205, -0.53846;
           -0.07692,  0.12308,  0.02564,  0.03077;
            0.35897,  0.35897,  0.43590,  0.92308;
           -0.69231, -0.69231, -0.76923, -1.92308;
        ];

        assert_fuzzy_eq!(a.inverse(), inverse);
    }

    #[test]
    fn inverse_of_a_third_matrix() {
        let a = matrix![
            9,  3,  0,  9;
           -5, -2, -6, -3;
           -4,  9,  6,  4;
           -7,  6,  6,  2;
        ];

        let inverse = matrix![
           -0.04074, -0.07778,  0.14444, -0.22222;
           -0.07778,  0.03333,  0.36667, -0.33333;
           -0.02901, -0.14630, -0.10926,  0.12963;
            0.17778,  0.06667, -0.26667,  0.33333;
        ];

        assert_fuzzy_eq!(a.inverse(), inverse);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = matrix![
             3, -9,  7,  3;
             3, -8,  2, -9;
            -4,  4,  4,  1;
            -6,  5, -1,  1;
        ];

        let b = matrix![
             8,  2,  2,  2;
             3, -1,  7,  0;
             7,  0,  5,  4;
             6, -2,  0,  5;
        ];

        let c = a * b;

        assert_fuzzy_eq!(c * b.inverse(), a);
    }

    #[test]
    fn putting_it_all_together() {
        //Q: What happens when you invert the identity matrix?
        let identity = Matrix::identity();
        assert_fuzzy_eq!(identity.inverse(), identity);
        //A: It doesn't change

        //Q: What do you get when you multiply a matrix by it's inverse?
        let a = matrix![
             3, -9,  7,  3;
             3, -8,  2, -9;
            -4,  4,  4,  1;
            -6,  5, -1,  1;
        ];
        assert_fuzzy_eq!(a * a.inverse(), identity);
        //A: You get the identity matrix

        //Q: Is there any difference between the inverse of the transpose of a matrix, and the transpose of the inverse?
        assert_fuzzy_eq!(a.transpose().inverse(), a.inverse().transpose());
        //A: No

        //Q: Given multiplying the identity matrix by a tuple gives the tuple unchanged, what happens when you change
        //any single element of the identity matrix prior to multiplying?
        let t = Tuple::from((1, 2, 3, 1));

        let mut m = identity;
        m[0][0] = 5.0;
        //A: The tuple changes
        println!("{:?}", m * t);
    }
}
