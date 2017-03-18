use num::Zero;
use std::convert::From;
use math::scalar::*;
use std::ops::*;

type Matrix2x2Array = [[FloatScalar; 2]; 2];
type Matrix3x3Array = [[FloatScalar; 3]; 3];
type Matrix4x4Array = [[FloatScalar; 4]; 4];

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Matrix2x2 {
    m: Matrix2x2Array,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Matrix3x3 {
    m: Matrix3x3Array,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Matrix4x4 {
    m: Matrix4x4Array,
}


//
// Matrix2x2
//
pub trait Matrix where 
    Self: Zero,
    Self: Index<usize, Output = [FloatScalar]>,
    Self: IndexMut<usize, Output = [FloatScalar]>,
    Self: Add<Output = Self>,
    Self: Sub<Output = Self>,
    Self: Mul<Output = Self>,
    Self: Mul<FloatScalar, Output = Self>,
    Self: Div<FloatScalar, Output = Self> {
    fn identity() -> Self;

    fn tranpose(&self) -> Self;

    fn inverse(&self) -> Option<Self>;

    fn determinant(&self) -> FloatScalar;
}

impl Matrix2x2 {
    pub fn new(t00: FloatScalar, t01: FloatScalar, t10: FloatScalar, t11: FloatScalar) -> Matrix2x2 {
        Matrix2x2 {
            m : [
                [t00, t01],
                [t10, t11],
            ],
        }
    }

    pub fn minor(&self, i: usize, j: usize) -> FloatScalar {
        if i > 1 || j > 1 {
            panic!("index '{}, {}' out of bounds", i, j)
        }

        self.m[(i + 1) % 2][(j + 1) % 2]
    }
}

impl From<Matrix2x2Array> for Matrix2x2 {
    fn from(m: Matrix2x2Array) -> Matrix2x2 {
        Matrix2x2 {
            m:  m,
        }
    }
}

impl Zero for Matrix2x2 {
    fn zero() -> Matrix2x2 {
        Matrix2x2 {
            m: [
                [0.0, 0.0],
                [0.0, 0.0],
            ]
        }
    }

    fn is_zero(&self) -> bool {
        *self == Matrix2x2::zero()
    }
}

impl Add for Matrix2x2 {
    type Output = Matrix2x2;

    fn add(self, m: Matrix2x2) -> Matrix2x2 {
        Matrix2x2::new(
            self[0][0] + m[0][0], self[0][1] + m[0][1],
            self[1][0] + m[1][0], self[1][1] + m[1][1])
    }
}

impl Sub for Matrix2x2 {
    type Output = Matrix2x2;

    fn sub(self, m: Matrix2x2) -> Matrix2x2 {
        Matrix2x2::new(
            self[0][0] - m[0][0], self[0][1] - m[0][1],
            self[1][0] - m[1][0], self[1][1] - m[1][1])
    }
}

impl Mul<FloatScalar> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, t: FloatScalar) -> Matrix2x2 {
        Matrix2x2::new(
            self[0][0] * t, self[0][1] * t,
            self[1][0] * t, self[1][1] * t)
    }
}

impl Div<FloatScalar> for Matrix2x2 {
    type Output = Matrix2x2;

    fn div(self, t: FloatScalar) -> Matrix2x2 {
        Matrix2x2::new(
            self[0][0] / t, self[0][1] / t,
            self[1][0] / t, self[1][1] / t)
    }
}

impl Mul for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, m: Matrix2x2) -> Matrix2x2 {
        Matrix2x2::new(
            self[0][0] * m[0][0] + self[0][1] * m[1][0],
            self[0][0] * m[0][1] + self[0][1] * m[1][1],

            self[1][0] * m[0][0] + self[1][1] * m[1][0],
            self[1][0] * m[0][1] + self[1][1] * m[1][1])
    }
}

impl Index<usize> for Matrix2x2 {
    type Output = [FloatScalar];

    fn index(&self, index: usize) -> &[FloatScalar] {
        &self.m[index]
    }
}

impl IndexMut<usize> for Matrix2x2 {
    fn index_mut(&mut self, index: usize) -> &mut [FloatScalar] {
        &mut self.m[index]
    }
}

impl Matrix for Matrix2x2 {
    fn identity() -> Matrix2x2 {
        Matrix2x2 {
            m: [
                [1.0, 0.0],
                [0.0, 1.0],
            ]
        }
    }

    fn tranpose(&self) -> Matrix2x2 {
        Matrix2x2::new(self[0][0], self[1][0],
                       self[0][1], self[1][1])
    }

    fn inverse(&self) -> Option<Matrix2x2> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            let inv_det = 1.0 / det;
            Some(Matrix2x2 {
                m: [
                    [self.m[1][1] * inv_det, -self.m[0][1] * inv_det],
                    [-self.m[1][0] * inv_det, self.m[0][0] * inv_det],
                ],
            })
        }
    }

    fn determinant(&self) -> FloatScalar {
        self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]
    }
}


//
// Matrix3x3
//
impl Matrix3x3 {
    pub fn new(t00: FloatScalar, t01: FloatScalar, t02: FloatScalar,
               t10: FloatScalar, t11: FloatScalar, t12: FloatScalar,
               t20: FloatScalar, t21: FloatScalar, t22: FloatScalar) -> Matrix3x3 {
        Matrix3x3 {
            m: [
                [t00, t01, t02],
                [t10, t11, t12],
                [t20, t21, t22],
            ],
        }
    }

    pub fn minor(&self, i: usize, j: usize) -> Matrix2x2 {
        if i > 2 || j > 2 {
            panic!("index '{}, {}' out of bounds", i, j)
        }

        let fst_row = if i == 0 { 1 } else { 0 };
        let snd_row = if i == 1 { 2 } else { fst_row + 1 };
        let fst_col = if j == 0 { 1 } else { 0 };
        let snd_col = if j == 1 { 2 } else { fst_col + 1 };

        Matrix2x2::new(self.m[fst_row][fst_col], self.m[fst_row][snd_col],
                       self.m[snd_row][fst_col], self.m[snd_row][snd_col])
    }
}

impl From<Matrix3x3Array> for Matrix3x3 {
    fn from(m: Matrix3x3Array) -> Matrix3x3 {
        Matrix3x3 {
            m:  m,
        }
    }
}

impl Zero for Matrix3x3 {
    fn zero() -> Matrix3x3 {
        Matrix3x3 {
            m: [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ]
        }
    }

    fn is_zero(&self) -> bool {
        *self == Matrix3x3::zero()
    }
}

impl Add for Matrix3x3 {
    type Output = Matrix3x3;

    fn add(self, m: Matrix3x3) -> Matrix3x3 {
        Matrix3x3::new(
            self[0][0] + m[0][0], self[0][1] + m[0][1], self[0][3] + m[0][3],
            self[1][0] + m[1][0], self[1][1] + m[1][1], self[1][3] + m[1][3],
            self[2][0] + m[2][0], self[2][1] + m[2][1], self[2][3] + m[2][3])
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;

    fn sub(self, m: Matrix3x3) -> Matrix3x3 {
        Matrix3x3::new(
            self[0][0] - m[0][0], self[0][1] - m[0][1], self[0][3] - m[0][3],
            self[1][0] - m[1][0], self[1][1] - m[1][1], self[1][3] - m[1][3],
            self[2][0] - m[2][0], self[2][1] - m[2][1], self[2][3] - m[2][3])
    }
}

impl Mul<FloatScalar> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, t: FloatScalar) -> Matrix3x3 {
        Matrix3x3::new(
            self[0][0] * t, self[0][1] * t, self[0][3] * t,
            self[1][0] * t, self[1][1] * t, self[1][3] * t,
            self[2][0] * t, self[2][1] * t, self[2][3] * t)
    }
}

impl Div<FloatScalar> for Matrix3x3 {
    type Output = Matrix3x3;

    fn div(self, t: FloatScalar) -> Matrix3x3 {
        Matrix3x3::new(
            self[0][0] / t, self[0][1] / t, self[0][3] / t,
            self[1][0] / t, self[1][1] / t, self[1][3] / t,
            self[2][0] / t, self[2][1] / t, self[2][3] / t)
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, m: Matrix3x3) -> Matrix3x3 {
        Matrix3x3::new(
            self[0][0] * m[0][0] + self[0][1] * m[1][0] + self[0][2] * m[2][0],
            self[0][0] * m[0][1] + self[0][1] * m[1][1] + self[0][2] * m[2][1],
            self[0][0] * m[0][2] + self[0][1] * m[1][2] + self[0][2] * m[2][2],

            self[1][0] * m[0][0] + self[1][1] * m[1][0] + self[1][2] * m[2][0],
            self[1][0] * m[0][1] + self[1][1] * m[1][1] + self[1][2] * m[2][1],
            self[1][0] * m[0][2] + self[1][1] * m[1][2] + self[1][2] * m[2][2],

            self[2][0] * m[0][0] + self[2][1] * m[1][0] + self[2][2] * m[2][0],
            self[2][0] * m[0][1] + self[2][1] * m[1][1] + self[2][2] * m[2][1],
            self[2][0] * m[0][2] + self[2][1] * m[1][2] + self[2][2] * m[2][2])
    }
}

impl Index<usize> for Matrix3x3 {
    type Output = [FloatScalar];

    fn index(&self, index: usize) -> &[FloatScalar] {
        &self.m[index]
    }
}

impl IndexMut<usize> for Matrix3x3 {
    fn index_mut(&mut self, index: usize) -> &mut [FloatScalar] {
        &mut self.m[index]
    }
}

impl Matrix for Matrix3x3 {
    fn identity() -> Matrix3x3 {
        Matrix3x3 {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    fn tranpose(&self) -> Matrix3x3 {
        Matrix3x3::new(
            self[0][0], self[1][0], self[2][0],
            self[0][1], self[1][1], self[2][1],
            self[0][2], self[1][2], self[2][2])
    }

    fn inverse(&self) -> Option<Matrix3x3> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            let inv_det = 1.0 / det;
            Some(Matrix3x3::new(
                self.minor(0, 0).determinant() * inv_det,
                self.minor(0, 1).determinant() * inv_det,
                self.minor(0, 2).determinant() * inv_det,

                self.minor(1, 0).determinant() * inv_det,
                self.minor(1, 1).determinant() * inv_det,
                self.minor(1, 2).determinant() * inv_det,

                self.minor(2, 0).determinant() * inv_det,
                self.minor(2, 1).determinant() * inv_det,
                self.minor(2, 2).determinant() * inv_det
            ))
        }
    }

    fn determinant(&self) -> FloatScalar {
        self.m[0][0] * self.minor(0, 0).determinant()
            - self.m[0][1] * self.minor(0, 1).determinant()
            + self.m[0][2] * self.minor(0, 2).determinant()
    }
}

//
// Matrix4x4
//
impl Matrix4x4 {
    pub fn new(t00: FloatScalar, t01: FloatScalar, t02: FloatScalar, t03: FloatScalar,
               t10: FloatScalar, t11: FloatScalar, t12: FloatScalar, t13: FloatScalar,
               t20: FloatScalar, t21: FloatScalar, t22: FloatScalar, t23: FloatScalar,
               t30: FloatScalar, t31: FloatScalar, t32: FloatScalar, t33: FloatScalar) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                [t00, t01, t02, t03],
                [t10, t11, t12, t13],
                [t20, t21, t22, t23],
                [t30, t31, t32, t33],
            ],
        }
    }

    pub fn minor(&self, i: usize, j: usize) -> Matrix3x3 {
        if i > 3 || j > 3 {
            panic!("index '{}, {}' out of bounds", i, j)
        }

        let fst_row = if i == 0 { 1 } else { 0 };
        let snd_row = if i == 1 { 2 } else { fst_row + 1 };
        let trd_row = if i == 2 { 3 } else { snd_row + 1 };
        let fst_col = if j == 0 { 1 } else { 0 };
        let snd_col = if j == 1 { 2 } else { fst_col + 1 };
        let trd_col = if j == 2 { 3 } else { snd_col + 1 };

        Matrix3x3::new(self.m[fst_row][fst_col], self.m[fst_row][snd_col], self.m[fst_row][trd_col],
                       self.m[snd_row][fst_col], self.m[snd_row][snd_col], self.m[snd_row][trd_col],
                       self.m[trd_row][fst_col], self.m[trd_row][snd_col], self.m[trd_row][trd_col])
    }
}

impl From<Matrix4x4Array> for Matrix4x4 {
    fn from(m: Matrix4x4Array) -> Matrix4x4 {
        Matrix4x4 {
            m:  m,
        }
    }
}

impl Zero for Matrix4x4 {
    fn zero() -> Matrix4x4 {
        Matrix4x4 {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
        }
    }

    fn is_zero(&self) -> bool {
        *self == Matrix4x4::zero()
    }
}

impl Add for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, m: Matrix4x4) -> Matrix4x4 {
        Matrix4x4::new(
            self[0][0] + m[0][0], self[0][1] + m[0][1], self[0][3] + m[0][3], self[0][3] + m[0][3],
            self[1][0] + m[1][0], self[1][1] + m[1][1], self[1][3] + m[1][3], self[1][3] + m[1][3],
            self[2][0] + m[2][0], self[2][1] + m[2][1], self[2][3] + m[2][3], self[2][3] + m[2][3],
            self[3][0] + m[3][0], self[3][1] + m[3][1], self[3][3] + m[3][3], self[3][3] + m[3][3])
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, m: Matrix4x4) -> Matrix4x4 {
        Matrix4x4::new(
            self[0][0] - m[0][0], self[0][1] - m[0][1], self[0][3] - m[0][3], self[0][3] - m[0][3],
            self[1][0] - m[1][0], self[1][1] - m[1][1], self[1][3] - m[1][3], self[1][3] - m[1][3],
            self[2][0] - m[2][0], self[2][1] - m[2][1], self[2][3] - m[2][3], self[2][3] - m[2][3],
            self[3][0] - m[3][0], self[3][1] - m[3][1], self[3][3] - m[3][3], self[3][3] - m[3][3])
    }
}

impl Mul<FloatScalar> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, t: FloatScalar) -> Matrix4x4 {
        Matrix4x4::new(
            self[0][0] * t, self[0][1] * t, self[0][3] * t, self[0][3] * t,
            self[1][0] * t, self[1][1] * t, self[1][3] * t, self[1][3] * t,
            self[2][0] * t, self[2][1] * t, self[2][3] * t, self[2][3] * t,
            self[3][0] * t, self[3][1] * t, self[3][3] * t, self[3][3] * t)
    }
}

impl Div<FloatScalar> for Matrix4x4 {
    type Output = Matrix4x4;

    fn div(self, t: FloatScalar) -> Matrix4x4 {
        Matrix4x4::new(
            self[0][0] / t, self[0][1] / t, self[0][3] / t, self[0][3] / t,
            self[1][0] / t, self[1][1] / t, self[1][3] / t, self[1][3] / t,
            self[2][0] / t, self[2][1] / t, self[2][3] / t, self[2][3] / t,
            self[3][0] / t, self[3][1] / t, self[3][3] / t, self[3][3] / t)
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, m: Matrix4x4) -> Matrix4x4 {
        Matrix4x4::new(
            self[0][0] * m[0][0] + self[0][1] * m[1][0] + self[0][2] * m[2][0] + self[0][3] * m[3][0],
            self[0][0] * m[0][1] + self[0][1] * m[1][1] + self[0][2] * m[2][1] + self[0][3] * m[3][1],
            self[0][0] * m[0][2] + self[0][1] * m[1][2] + self[0][2] * m[2][2] + self[0][3] * m[3][2],
            self[0][0] * m[0][3] + self[0][1] * m[1][3] + self[0][2] * m[2][3] + self[0][3] * m[3][3],

            self[1][0] * m[0][0] + self[1][1] * m[1][0] + self[1][2] * m[2][0] + self[1][3] * m[3][0],
            self[1][0] * m[0][1] + self[1][1] * m[1][1] + self[1][2] * m[2][1] + self[1][3] * m[3][1],
            self[1][0] * m[0][2] + self[1][1] * m[1][2] + self[1][2] * m[2][2] + self[1][3] * m[3][2],
            self[1][0] * m[0][3] + self[1][1] * m[1][3] + self[1][2] * m[2][3] + self[1][3] * m[3][3],

            self[2][0] * m[0][0] + self[2][1] * m[1][0] + self[2][2] * m[2][0] + self[2][3] * m[3][0],
            self[2][0] * m[0][1] + self[2][1] * m[1][1] + self[2][2] * m[2][1] + self[2][3] * m[3][1],
            self[2][0] * m[0][2] + self[2][1] * m[1][2] + self[2][2] * m[2][2] + self[2][3] * m[3][2],
            self[2][0] * m[0][3] + self[2][1] * m[1][3] + self[2][2] * m[2][3] + self[2][3] * m[3][3],

            self[3][0] * m[0][0] + self[3][1] * m[1][0] + self[3][2] * m[2][0] + self[3][3] * m[3][0],
            self[3][0] * m[0][1] + self[3][1] * m[1][1] + self[3][2] * m[2][1] + self[3][3] * m[3][1],
            self[3][0] * m[0][2] + self[3][1] * m[1][2] + self[3][2] * m[2][2] + self[3][3] * m[3][2],
            self[3][0] * m[0][3] + self[3][1] * m[1][3] + self[3][2] * m[2][3] + self[3][3] * m[3][3])
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = [FloatScalar];

    fn index(&self, index: usize) -> &[FloatScalar] {
        &self.m[index]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut [FloatScalar] {
        &mut self.m[index]
    }
}

impl Matrix for Matrix4x4 {
    fn identity() -> Matrix4x4 {
        Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    fn tranpose(&self) -> Matrix4x4 {
        Matrix4x4::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3])
    }

    fn inverse(&self) -> Option<Matrix4x4> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            let inv_det = 1.0 / det;
            Some(Matrix4x4::new(
                self.minor(0, 0).determinant() * inv_det,
                self.minor(0, 1).determinant() * inv_det,
                self.minor(0, 2).determinant() * inv_det,
                self.minor(0, 3).determinant() * inv_det,

                self.minor(1, 0).determinant() * inv_det,
                self.minor(1, 1).determinant() * inv_det,
                self.minor(1, 2).determinant() * inv_det,
                self.minor(1, 3).determinant() * inv_det,

                self.minor(2, 0).determinant() * inv_det,
                self.minor(2, 1).determinant() * inv_det,
                self.minor(2, 2).determinant() * inv_det,
                self.minor(2, 3).determinant() * inv_det,

                self.minor(3, 0).determinant() * inv_det,
                self.minor(3, 1).determinant() * inv_det,
                self.minor(3, 2).determinant() * inv_det,
                self.minor(3, 3).determinant() * inv_det,
            ))
        }
    }

    fn determinant(&self) -> FloatScalar {
        self.m[0][0] * self.minor(0, 0).determinant()
            - self.m[0][1] * self.minor(0, 1).determinant()
            + self.m[0][2] * self.minor(0, 2).determinant()
            - self.m[0][3] * self.minor(0, 3).determinant()
    }
}
