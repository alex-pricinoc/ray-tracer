#![allow(dead_code)]

use crate::FuzzyEq;
use crate::Tuple;
use crate::F;
use std::fmt;
use std::ops::{Index, IndexMut, Mul};

#[derive(Copy, Clone)]
pub struct Matrix<const D: usize>(pub [[F; D]; D]);

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
    #[must_use]
    pub fn new() -> Self {
        Self([[0.0; D]; D])
    }

    #[must_use]
    pub fn size(&self) -> usize {
        D
    }

    #[must_use]
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
    #[must_use]
    pub fn determinant(&self) -> F {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    #[must_use]
    pub fn is_invertible(&self) -> bool {
        self.determinant() == 0.0
    }
}

impl Matrix<3> {
    #[must_use]
    pub fn determinant(&self) -> F {
        let mut det = 0.0;

        for c in 0..self.size() {
            det += self[0][c] * self.cofactor(0, c);
        }

        det
    }

    #[must_use]
    pub fn minor(&self, row: usize, col: usize) -> F {
        self.submatrix(row, col).determinant()
    }

    #[must_use]
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

    #[must_use]
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
    #[must_use]
    pub fn identity() -> Self {
        matrix![
            1, 0, 0, 0;
            0, 1, 0, 0;
            0, 0, 1, 0;
            0, 0, 0, 1;
        ]
    }

    #[must_use]
    pub fn translation(x: impl Into<F>, y: impl Into<F>, z: impl Into<F>) -> Self {
        matrix![
            1, 0, 0, x.into();
            0, 1, 0, y.into();
            0, 0, 1, z.into();
            0, 0, 0, 1;
        ]
    }

    #[must_use]
    pub fn translate(self, x: impl Into<F>, y: impl Into<F>, z: impl Into<F>) -> Self {
        Self::translation(x, y, z) * self
    }

    #[must_use]
    pub fn scaling(x: impl Into<F>, y: impl Into<F>, z: impl Into<F>) -> Self {
        matrix![
            x.into(), 0, 0, 0;
            0, y.into(), 0, 0;
            0, 0, z.into(), 0;
            0, 0, 0, 1;
        ]
    }

    #[must_use]
    pub fn scale(self, x: impl Into<F>, y: impl Into<F>, z: impl Into<F>) -> Self {
        Self::scaling(x, y, z) * self
    }

    #[must_use]
    pub fn sheare(
        self,
        xy: impl Into<F>,
        xz: impl Into<F>,
        yx: impl Into<F>,
        yz: impl Into<F>,
        zx: impl Into<F>,
        zy: impl Into<F>,
    ) -> Self {
        Self::shearing(xy, xz, yx, yz, zx, zy) * self
    }

    #[must_use]
    pub fn rotation_x(r: F) -> Self {
        matrix![
          1,    0,       0,     0;
          0, r.cos(), -r.sin(), 0;
          0, r.sin(),  r.cos(), 0;
          0,    0,       0,     1;
        ]
    }

    #[must_use]
    pub fn rotate_x(self, r: F) -> Self {
        Self::rotation_x(r) * self
    }

    #[must_use]
    pub fn rotation_y(r: F) -> Self {
        matrix![
            r.cos(),  0, r.sin(), 0;
            0,        1, 0,       0;
            -r.sin(), 0, r.cos(), 0;
            0,        0, 0,       1;
        ]
    }

    #[must_use]
    pub fn rotate_y(self, r: F) -> Self {
        Self::rotation_y(r) * self
    }

    #[must_use]
    pub fn rotation_z(r: F) -> Self {
        matrix![
            r.cos(), -r.sin(), 0, 0;
            r.sin(),  r.cos(), 0, 0;
            0,         0,      1, 0;
            0,         0,      0, 1;
        ]
    }
    #[must_use]
    pub fn rotate_z(self, r: F) -> Self {
        Self::rotation_z(r) * self
    }

    pub fn shearing(
        xy: impl Into<F>,
        xz: impl Into<F>,
        yx: impl Into<F>,
        yz: impl Into<F>,
        zx: impl Into<F>,
        zy: impl Into<F>,
    ) -> Self {
        matrix![
            1, xy.into(), xz.into(), 0;
            yx.into(), 1, yz.into(), 0;
            zx.into(), zy.into(), 1, 0;
            0, 0, 0, 1;
        ]
    }

    #[must_use]
    pub fn determinant(&self) -> F {
        let mut det = 0.0;

        for c in 0..self.size() {
            det += self[0][c] * self.cofactor(0, c);
        }

        det
    }

    #[must_use]
    pub fn cofactor(&self, row: usize, col: usize) -> F {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    #[must_use]
    pub fn minor(&self, row: usize, col: usize) -> F {
        self.submatrix(row, col).determinant()
    }

    #[must_use]
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

    #[must_use]
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    #[must_use]
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
        write!(f, "{self}")
    }
}

impl<const D: usize> Default for Matrix<D> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

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

        assert_eq!(a.minor(1, 0), 25.0);
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
        assert_eq!(a.cofactor(1, 0), -25.0);
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
        assert_eq!(m * t, Tuple::from((5, 2, 3, 1)));
    }

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix::translation(5, -3, 2);
        let p = pt(-3, 4, 5);

        assert_eq!(transform * p, pt(2, 1, 7));
    }

    #[test]
    fn multipliying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::translation(5, -3, 2);
        let inv = transform.inverse();
        let p = pt(-3, 4, 5);

        assert_eq!(inv * p, pt(-8, 7, 3));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5, -3, 2);
        let v1 = v(-3, 4, 5);

        assert_eq!(transform * v1, v1);
    }

    #[test]
    fn scaling_matrix_applied_to_pt() {
        let transform = Matrix::scaling(2, 3, 4);
        let p = pt(-4, 6, 8);

        assert_eq!(transform * p, pt(-8, 18, 32));
    }

    #[test]
    fn scaling_matrix_applied_to_v() {
        let transform = Matrix::scaling(2, 3, 4);
        let v1 = v(-4, 6, 8);

        assert_eq!(transform * v1, v(-8, 18, 32));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix::scaling(2, 3, 4);
        let inv = transform.inverse();

        let v1 = v(-4, 6, 8);

        assert_eq!(inv * v1, v(-2, 2, 2));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix::scaling(-1, 1, 1);
        let p = pt(2, 3, 4);

        assert_fuzzy_eq!(transform * p, pt(-2, 3, 4));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = pt(0, 1, 0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);

        assert_fuzzy_eq!(
            half_quarter * p,
            pt(0, F::sqrt(2.0) / 2.0, F::sqrt(2.0) / 2.0)
        );

        assert_fuzzy_eq!(full_quarter * p, pt(0, 0, 1));
    }

    #[test]
    fn inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = pt(0, 1, 0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);

        let inv = half_quarter.inverse();

        assert_fuzzy_eq!(inv * p, pt(0, F::sqrt(2.0) / 2.0, -(F::sqrt(2.0)) / 2.0));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = pt(0, 0, 1);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);

        assert_fuzzy_eq!(
            half_quarter * p,
            pt(F::sqrt(2.0) / 2.0, 0, F::sqrt(2.0) / 2.0)
        );

        assert_fuzzy_eq!(full_quarter * p, pt(1, 0, 0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = pt(0, 1, 0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);

        assert_fuzzy_eq!(
            half_quarter * p,
            pt(-F::sqrt(2.0) / 2.0, F::sqrt(2.0) / 2.0, 0)
        );

        assert_fuzzy_eq!(full_quarter * p, pt(-1, 0, 0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1, 0, 0, 0, 0, 0);
        let p = pt(2, 3, 4);
        assert_fuzzy_eq!(transform * p, pt(5, 3, 4));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0, 1, 0, 0, 0, 0);
        let p = pt(2, 3, 4);
        assert_fuzzy_eq!(transform * p, pt(6, 3, 4));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0, 0, 1, 0, 0, 0);
        let p = pt(2, 3, 4);
        assert_fuzzy_eq!(transform * p, pt(2, 5, 4));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0, 0, 0, 1, 0, 0);
        let p = pt(2, 3, 4);
        assert_fuzzy_eq!(transform * p, pt(2, 7, 4));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0, 0, 0, 0, 1, 0);
        let p = pt(2, 3, 4);
        assert_fuzzy_eq!(transform * p, pt(2, 3, 6));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0, 0, 0, 0, 0, 1);
        let p = pt(2, 3, 4);
        assert_fuzzy_eq!(transform * p, pt(2, 3, 7));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = pt(1, 0, 1);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5, 5, 5);
        let c = Matrix::translation(10, 5, 7);

        // rotation
        let p2 = a * p;
        assert_fuzzy_eq!(p2, pt(1, -1, 0));

        // scaling
        let p3 = b * p2;
        assert_fuzzy_eq!(p3, pt(5, -5, 0));

        // translation
        let p4 = c * p3;
        assert_fuzzy_eq!(p4, pt(15, 0, 7));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = pt(1, 0, 1);
        // let a = Matrix::rotation_x(PI / 2.0);
        // let b = Matrix::scaling(5, 5, 5);
        // let c = Matrix::translation(10, 5, 7);

        //chained transformations must be applied in reverse order
        // let t = c * b * a;

        // fluent API (aka consuming builder pattern)
        let t = Matrix::identity()
            .rotate_x(PI / 2.0)
            .scale(5, 5, 5)
            .translate(10, 5, 7);

        assert_fuzzy_eq!(t * p, pt(15, 0, 7));
    }
}
