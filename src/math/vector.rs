use num::{Zero, Signed};
use math::point::{Point2, Point3};
use math::normal::Normal3;
use std::convert::From;
use math::common::*;
use math::scalar::*;
use std::ops::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

//
// Vector3
//
impl <T: BaseNum> Vector3<T> {
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

    pub fn permute(&self, x: Dimension3, y: Dimension3, z: Dimension3) -> Vector3<T> {
        Vector3::new(self[x], self[y], self[z])
    }
}

impl <T: BaseNum> From<T> for Vector3<T> {
    fn from(s: T) -> Vector3<T> {
        Vector3::new(s, s, s)
    }
}

impl <T: BaseNum> From<Point3<T>> for Vector3<T> {
    fn from(p: Point3<T>) -> Vector3<T> {
        Vector3::new(p.x, p.y, p.z)
    }
}

impl <T: BaseNum> From<Normal3<T>> for Vector3<T> {
    fn from(n: Normal3<T>) -> Vector3<T> {
        Vector3::new(n.x, n.y, n.z)
    }
}

impl <T: BaseNum> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl <T: BaseNum> Index<Dimension3> for Vector3<T> {
    type Output = T;

    fn index(&self, index: Dimension3) -> &T {
        match index {
            Dimension3::X => &self.x,
            Dimension3::Y => &self.y,
            Dimension3::Z => &self.z,
        }
    }
}

impl <T: BaseNum> Zero for Vector3<T> {
    fn zero() -> Vector3<T> {
        Vector3::new(T::zero(), T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero() && self.z == T::zero()
    }
}

impl <T: BaseNum + Neg<Output = T>> Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Vector3<T> {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl <T: BaseNum> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl <T: BaseNum> AddAssign for Vector3<T> {
    fn add_assign(&mut self, other: Vector3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl <T: BaseNum> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl <T: BaseNum> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, other: Vector3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl <T: BaseNum> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, scalar: T) -> Vector3<T> {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl <T: BaseNum> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl <T: BaseNum> Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, scalar: T) -> Vector3<T> {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl <T: BaseNum> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl <T: BaseNum> ComponentWise for Vector3<T> {
    type Scalar = T;
    type Dimension = Dimension3;

    fn min_component(self) -> T {
        partial_min(self.x, partial_min(self.y, self.z))
    }

    fn max_component(self) -> T {
        partial_max(self.x, partial_max(self.y, self.z))
    }

    fn max_dimension(self) -> Dimension3 {
        if self.x > self.y && self.x > self.z {
            Dimension3::X
        } else if self.y > self.x && self.y > self.z {
            Dimension3::Y
        } else {
            Dimension3::Z
        }
    }

    fn min(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(partial_min(self.x, other.x), partial_min(self.y, other.y), partial_min(self.z, other.z))
    }

    fn max(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(partial_max(self.x, other.x), partial_max(self.y, other.y), partial_max(self.z, other.z))
    }
}

impl <T: BaseNum + Signed> ComponentWiseSigned for Vector3<T> {
    fn abs(self) -> Vector3<T> {
        Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl <T: BaseFloat> ComponentWiseFloat for Vector3<T> {
    fn floor(self) -> Vector3<T> {
        Vector3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    fn ceil(self) -> Vector3<T> {
        Vector3::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
}

impl <T: BaseNum> VectorSpace for Vector3<T> {
    type Scalar = T;
}

impl <T: BaseNum> CrossProduct for Vector3<T> {
    type CrossOutput = Vector3<T>;
    
    fn cross(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new((self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x))
    }
}

impl <T: BaseNum> CrossProduct<Normal3<T>> for Vector3<T> {
    type CrossOutput = Vector3<T>;
    
    fn cross(self, other: Normal3<T>) -> Vector3<T> {
        Vector3::new((self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x))
    }
}

impl <T: BaseNum> InnerProduct for Vector3<T> {
    fn dot(self, other: Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl <T: BaseNum> InnerProduct<Normal3<T>> for Vector3<T> {
    fn dot(self, other: Normal3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl <T: BaseFloat> InnerProductSpace for Vector3<T> {}

impl <T: BaseFloat> MetricSpace for Vector3<T> {
    type Scalar = T;

    fn distance_squared(self, other: Vector3<T>) -> T {
        (self - other).magnitude_squared()
    }
}

impl <T: BaseFloat> LinearInterpolate for Vector3<T> {
    type Scalar = T;
}


//
// Vector2
//
impl <T: BaseNum> Vector2<T> {
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

impl <T: BaseNum> From<T> for Vector2<T> {
    fn from(s: T) -> Vector2<T> {
        Vector2::new(s, s)
    }
}

impl <T: BaseNum> From<Point2<T>> for Vector2<T> {
    fn from(v: Point2<T>) -> Vector2<T> {
        Vector2::new(v.x, v.y)
    }
}

impl <T: BaseNum> Index<usize> for Vector2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl <T: BaseNum> Index<Dimension2> for Vector2<T> {
    type Output = T;

    fn index(&self, index: Dimension2) -> &T {
        match index {
            Dimension2::X => &self.x,
            Dimension2::Y => &self.y,
        }
    }
}

impl <T: BaseNum> Zero for Vector2<T> {
    fn zero() -> Vector2<T> {
        Vector2::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
    }
}

impl <T: BaseNum + Neg<Output = T>> Neg for Vector2<T> {
    type Output = Vector2<T>;

    fn neg(self) -> Vector2<T> {
        Vector2::new(-self.x, -self.y)
    }
}

impl <T: BaseNum> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x + other.x,self.y + other.y)
    }
}

impl <T: BaseNum> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Vector2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl <T: BaseNum> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl <T: BaseNum> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, other: Vector2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl <T: BaseNum> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, scalar: T) -> Vector2<T> {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl <T: BaseNum> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl <T: BaseNum> Div<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, scalar: T) -> Vector2<T> {
        Vector2::new(self.x / scalar, self.y / scalar)
    }
}

impl <T: BaseNum> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl <T: BaseNum> ComponentWise for Vector2<T> {
    type Scalar = T;
    type Dimension = Dimension2;

    fn min_component(self) -> T {
        partial_min(self.x, self.y)
    }

    fn max_component(self) -> T {
        partial_max(self.x, self.y)
    }

    fn max_dimension(self) -> Dimension2 {
        if self.x > self.y {
            Dimension2::X
        } else {
            Dimension2::Y
        }
    }

    fn min(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(partial_min(self.x, other.x), partial_min(self.y, other.y))
    }

    fn max(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(partial_max(self.x, other.x), partial_max(self.y, other.y))
    }
}

impl <T: BaseNum + Signed> ComponentWiseSigned for Vector2<T> {
    fn abs(self) -> Vector2<T> {
        Vector2::new(self.x.abs(), self.y.abs())
    }
}

impl <T: BaseFloat> ComponentWiseFloat for Vector2<T> {
    fn floor(self) -> Vector2<T> {
        Vector2::new(self.x.floor(), self.y.floor())
    }

    fn ceil(self) -> Vector2<T> {
        Vector2::new(self.x.ceil(), self.y.ceil())
    }
}

impl <T: BaseNum> VectorSpace for Vector2<T> {
    type Scalar = T;
}

impl <T: BaseNum> InnerProduct for Vector2<T> {
    fn dot(self, other: Vector2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl <T: BaseFloat> InnerProductSpace for Vector2<T> {}

impl <T: BaseFloat> MetricSpace for Vector2<T> {
    type Scalar = T;

    fn distance_squared(self, other: Vector2<T>) -> T {
        (self - other).magnitude_squared()
    }
}

impl <T: BaseFloat> LinearInterpolate for Vector2<T> {
    type Scalar = T;
}

pub fn vec2<T: BaseNum>(x: T, y: T) -> Vector2<T> {
    Vector2::new(x, y)
}

pub fn vec3<T: BaseNum>(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::new(x, y, z)
}

pub fn coordinate_system<T: BaseFloat>(v1: Vector3<T>) -> (Vector3<T>, Vector3<T>) {
    let v2 = if v1.x.abs() > v1.y.abs() {
        Vector3::new(-v1.z, T::zero(), v1.x).normalize()
    } else {
        Vector3::new(T::zero(), v1.z, -v1.y).normalize()
    };
    (v2, v1.cross(v2))
}

pub type Vector3i = Vector3<IntScalar>;
pub type Vector3f = Vector3<FloatScalar>;
pub type Vector2i = Vector2<IntScalar>;
pub type Vector2f = Vector2<FloatScalar>;