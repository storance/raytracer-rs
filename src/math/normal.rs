use num::Zero;
use math::vector::Vector3;

use std::convert::From;
use math::common::*;
use math::scalar::*;
use std::ops::*;

#[derive(PartialEq, Copy, Clone)]
pub struct Normal3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T: BaseNum> Normal3<T> {
    pub fn new(x : T, y : T, z: T) -> Normal3<T> {
        Normal3 {
            x: x,
            y: y,
            z: z
        }
    }
}

impl <T: BaseNum> From<Vector3<T>> for Normal3<T> {
    fn from(v: Vector3<T>) -> Normal3<T> {
        Normal3::new(v.x, v.y, v.z)
    }
}

impl <T: BaseNum> Zero for Normal3<T> {
    fn zero() -> Normal3<T> {
        Normal3::new(T::zero(), T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero() && self.z == T::zero()
    }
}

impl <T: BaseNum + Neg<Output = T>> Neg for Normal3<T> {
    type Output = Normal3<T>;

    fn neg(self) -> Normal3<T> {
        Normal3::new(-self.x, -self.y, -self.z)
    }
}

impl <T: BaseNum> Add for Normal3<T> {
    type Output = Normal3<T>;

    fn add(self, other: Normal3<T>) -> Normal3<T> {
        Normal3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl <T: BaseNum> AddAssign for Normal3<T> {
    fn add_assign(&mut self, other: Normal3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl <T: BaseNum> Sub for Normal3<T> {
    type Output = Normal3<T>;

    fn sub(self, other: Normal3<T>) -> Normal3<T> {
        Normal3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl <T: BaseNum> SubAssign for Normal3<T> {
    fn sub_assign(&mut self, other: Normal3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl <T: BaseNum> Mul<T> for Normal3<T> {
    type Output = Normal3<T>;

    fn mul(self, scalar: T) -> Normal3<T> {
        Normal3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl <T: BaseNum> MulAssign<T> for Normal3<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl <T: BaseNum> Div<T> for Normal3<T> {
    type Output = Normal3<T>;

    fn div(self, scalar: T) -> Normal3<T> {
        Normal3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl <T: BaseNum> DivAssign<T> for Normal3<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl <T: BaseNum> VectorSpace for Normal3<T> {
    type Scalar = T;
}

impl <T: BaseNum> CrossProduct<Vector3<T>> for Normal3<T> {
    type CrossOutput = Vector3<T>;
    
    fn cross(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new((self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x))
    }
}

impl <T: BaseNum> InnerProduct for Normal3<T> {
    fn dot(self, other: Normal3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl <T: BaseNum> InnerProduct<Vector3<T>> for Normal3<T> {
    fn dot(self, other: Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl <T: BaseFloat> InnerProductSpace for Normal3<T> {}

pub type Normal3f = Normal3<FloatScalar>;