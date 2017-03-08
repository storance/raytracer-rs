use math::scalar;
use num;
use num::One;
use std::ops;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub trait VectorSpace: Copy + Clone where
    Self: num::Zero,
    Self: ops::Add<Self, Output = Self>,
    Self: ops::Sub<Self, Output = Self>,
    Self: ops::Mul<<Self as VectorSpace>::Scalar, Output = Self>,
    Self: ops::Div<<Self as VectorSpace>::Scalar, Output = Self> {
    type Scalar: scalar::BaseNum;   
}

pub trait InnerSpace: VectorSpace where 
    <Self as VectorSpace>::Scalar: scalar::BaseFloat {
    fn dot(self, other: Self) -> Self::Scalar;

    fn magnitude(self) -> Self::Scalar {
        num::Float::sqrt(self.magnitude_squared())
    }

    fn magnitude_squared(self) -> Self::Scalar {
        Self::dot(self, self)
    }

    fn normalize(self) -> Self {
        self * (Self::Scalar::one() / self.magnitude())
    }

    fn lerp(self, other: Self, amount: Self::Scalar) -> Self {
        self + ((other - self) * amount)
    }
}

//
// Vector3
//
impl <T: scalar::BaseNum> Vector3<T> {
    pub fn new(x : T, y : T, z: T) -> Vector3<T> {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn from_value(s : T) -> Vector3<T> {
        Vector3::new(s, s, s)
    }

    pub fn unit_x() -> Vector3<T> {
        Vector3::new(T::one(), T::zero(), T::zero())
    }

    pub fn unit_y() -> Vector3<T> {
        Vector3::new(T::zero(), T::one(), T::zero())
    }

    pub fn unit_z() -> Vector3<T> {
        Vector3::new(T::zero(), T::zero(), T::one())
    }

    fn cross(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new((self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x))
    }
}

impl <T: scalar::BaseNum + ops::Neg<Output = T>> ops::Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Vector3<T> {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl <T: scalar::BaseNum + num::Signed> Vector3<T> {
    fn abs(&self) -> Vector3<T> {
        Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl <T: scalar::BaseNum> num::Zero for Vector3<T> {
    fn zero() -> Vector3<T> {
        Vector3::new(T::zero(), T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero() && self.z == T::zero()
    }
}

impl <T: scalar::BaseNum> ops::Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl <T: scalar::BaseNum> ops::AddAssign for Vector3<T> {
    fn add_assign(&mut self, other: Vector3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;

    }
}

impl <T: scalar::BaseNum> ops::Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl <T: scalar::BaseNum> ops::SubAssign for Vector3<T> {
    fn sub_assign(&mut self, other: Vector3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl <T: scalar::BaseNum> ops::Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, scalar: T) -> Vector3<T> {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl <T: scalar::BaseNum> ops::MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl <T: scalar::BaseNum> ops::Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, scalar: T) -> Vector3<T> {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl <T: scalar::BaseNum> ops::DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl <T: scalar::BaseNum> VectorSpace for Vector3<T> {
    type Scalar = T;
}

impl <T: scalar::BaseFloat> InnerSpace for Vector3<T> {
    fn dot(self, other: Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

//
// Vector2
//
impl <T: scalar::BaseNum> Vector2<T> {
    pub fn new(x : T, y : T) -> Vector2<T> {
        Vector2 {
            x: x,
            y: y
        }
    }

    pub fn from_value(s : T) -> Vector2<T> {
        Vector2::new(s, s)
    }

    pub fn unit_x() -> Vector2<T> {
        Vector2::new(T::one(), T::zero())
    }

    pub fn unit_y() -> Vector2<T> {
        Vector2::new(T::zero(), T::one())
    }
}

impl <T: scalar::BaseNum + ops::Neg<Output = T>> ops::Neg for Vector2<T> {
    type Output = Vector2<T>;

    fn neg(self) -> Vector2<T> {
        Vector2::new(-self.x, -self.y)
    }
}

impl <T: scalar::BaseNum + num::Signed> Vector2<T> {
    fn abs(&self) -> Vector2<T> {
        Vector2::new(self.x.abs(), self.y.abs())
    }
}

impl <T: scalar::BaseNum> num::Zero for Vector2<T> {
    fn zero() -> Vector2<T> {
        Vector2::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
    }
}

impl <T: scalar::BaseNum> ops::Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x + other.x,self.y + other.y)
    }
}

impl <T: scalar::BaseNum> ops::AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Vector2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl <T: scalar::BaseNum> ops::Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl <T: scalar::BaseNum> ops::SubAssign for Vector2<T> {
    fn sub_assign(&mut self, other: Vector2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl <T: scalar::BaseNum> ops::Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, scalar: T) -> Vector2<T> {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl <T: scalar::BaseNum> ops::MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl <T: scalar::BaseNum> ops::Div<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, scalar: T) -> Vector2<T> {
        Vector2::new(self.x / scalar, self.y / scalar)
    }
}

impl <T: scalar::BaseNum> ops::DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl <T: scalar::BaseNum> VectorSpace for Vector2<T> {
    type Scalar = T;
}

impl <T: scalar::BaseFloat> InnerSpace for Vector2<T> {
    fn dot(self, other: Vector2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

pub fn vec2<T: scalar::BaseNum>(x: T, y: T) -> Vector2<T> {
    Vector2::new(x, y)
}

pub fn vec3<T: scalar::BaseNum>(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::new(x, y, z)
}

pub type Vector3i = Vector3<scalar::IntScalar>;
pub type Vector3f = Vector3<scalar::FloatScalar>;
pub type Vector2i = Vector2<scalar::IntScalar>;
pub type Vector2f = Vector2<scalar::FloatScalar>;