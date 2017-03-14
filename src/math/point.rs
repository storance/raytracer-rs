use num::{Zero, Signed};
use math::vector::{Vector2, Vector3};
use math::common::*;
use math::scalar::*;
use std::convert::From;
use std::ops::*;

#[derive(PartialEq, Copy, Clone)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T: BaseNum> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn permute(&self, x: Dimension3, y: Dimension3, z: Dimension3) -> Point3<T> {
        Point3::new(self[x], self[y], self[z])
    }
}

impl <T: BaseNum> From<T> for Point3<T> {
    fn from(s: T) -> Point3<T> {
        Point3::new(s, s, s)
    }
}

impl <T: BaseNum> From<Vector3<T>> for Point3<T> {
    fn from(v: Vector3<T>) -> Point3<T> {
        Point3::new(v.x, v.y, v.z)
    }
}

impl <T: BaseNum> Index<usize> for Point3<T> {
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

impl <T: BaseNum> Index<Dimension3> for Point3<T> {
    type Output = T;

    fn index(&self, index: Dimension3) -> &T {
        match index {
            Dimension3::X => &self.x,
            Dimension3::Y => &self.y,
            Dimension3::Z => &self.z,
        }
    }
}

impl <T: BaseNum> Zero for Point3<T> {
    fn zero() -> Point3<T> {
        Point3::new(T::zero(), T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero() && self.z == T::zero()
    }
}

impl <T: BaseNum + Neg<Output = T>> Neg for Point3<T> {
    type Output = Point3<T>;

    fn neg(self) -> Point3<T> {
        Point3::new(-self.x, -self.y, -self.z)
    }
}

impl <T: BaseNum> Add for Point3<T> {
    type Output = Point3<T>;

    fn add(self, other: Point3<T>) -> Point3<T> {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl <T: BaseNum> AddAssign for Point3<T> {
    fn add_assign(&mut self, other: Point3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl <T: BaseNum> Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, other: Vector3<T>) -> Point3<T> {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl <T: BaseNum> AddAssign<Vector3<T>> for Point3<T> {
    fn add_assign(&mut self, other: Vector3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl <T: BaseNum> Sub for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: Point3<T>) -> Vector3<T> {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl <T: BaseNum> Sub<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn sub(self, other: Vector3<T>) -> Point3<T> {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl <T: BaseNum> SubAssign<Vector3<T>> for Point3<T> {
    fn sub_assign(&mut self, other: Vector3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl <T: BaseNum> Mul<T> for Point3<T> {
    type Output = Point3<T>;

    fn mul(self, scalar: T) -> Point3<T> {
        Point3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl <T: BaseNum> MulAssign<T> for Point3<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl <T: BaseNum> Div<T> for Point3<T> {
    type Output = Point3<T>;

    fn div(self, scalar: T) -> Point3<T> {
        Point3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl <T: BaseNum> DivAssign<T> for Point3<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl <T: BaseNum> ComponentWise for Point3<T> {
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

    fn min(self, other: Point3<T>) -> Point3<T> {
        Point3::new(partial_min(self.x, other.x), partial_min(self.y, other.y), partial_min(self.z, other.z))
    }

    fn max(self, other: Point3<T>) -> Point3<T> {
        Point3::new(partial_max(self.x, other.x), partial_max(self.y, other.y), partial_max(self.z, other.z))
    }
}

impl <T: BaseNum + Signed> ComponentWiseSigned for Point3<T> {
    fn abs(self) -> Point3<T> {
        Point3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl <T: BaseFloat> ComponentWiseFloat for Point3<T> {
    fn floor(self) -> Point3<T> {
        Point3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    fn ceil(self) -> Point3<T> {
        Point3::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
}

impl <T: BaseFloat> MetricSpace for Point3<T> {
    type Scalar = T;

    fn distance_squared(self, other: Point3<T>) -> T {
        (self - other).magnitude_squared()
    }
}

impl <T: BaseFloat> LinearInterpolate for Point3<T> {
    type Scalar = T;
}

//
// Point2
//
impl <T: BaseNum> Point2<T> {
    fn new(x: T, y: T) -> Point2<T> {
        Point2 {
            x: x,
            y: y,
        }
    }
}

impl <T: BaseNum> From<T> for Point2<T> {
    fn from(s: T) -> Point2<T> {
        Point2::new(s, s)
    }
}

impl <T: BaseNum> From<Vector3<T>> for Point2<T> {
    fn from(v: Vector3<T>) -> Point2<T> {
        Point2::new(v.x, v.y)
    }
}

impl <T: BaseNum> From<Point3<T>> for Point2<T> {
    fn from(p: Point3<T>) -> Point2<T> {
        Point2::new(p.x, p.y)
    }
}

impl <T: BaseNum> From<Vector2<T>> for Point2<T> {
    fn from(v: Vector2<T>) -> Point2<T> {
        Point2::new(v.x, v.y)
    }
}

impl <T: BaseNum> Index<usize> for Point2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl <T: BaseNum> Index<Dimension2> for Point2<T> {
    type Output = T;

    fn index(&self, index: Dimension2) -> &T {
        match index {
            Dimension2::X => &self.x,
            Dimension2::Y => &self.y,
        }
    }
}

impl <T: BaseNum> Zero for Point2<T> {
    fn zero() -> Point2<T> {
        Point2::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
    }
}

impl <T: BaseNum + Neg<Output = T>> Neg for Point2<T> {
    type Output = Point2<T>;

    fn neg(self) -> Point2<T> {
        Point2::new(-self.x, -self.y)
    }
}

impl <T: BaseNum> Add for Point2<T> {
    type Output = Point2<T>;

    fn add(self, other: Point2<T>) -> Point2<T> {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}

impl <T: BaseNum> AddAssign for Point2<T> {
    fn add_assign(&mut self, other: Point2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl <T: BaseNum> Add<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, other: Vector2<T>) -> Point2<T> {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}

impl <T: BaseNum> AddAssign<Vector2<T>> for Point2<T> {
    fn add_assign(&mut self, other: Vector2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl <T: BaseNum> Sub for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: Point2<T>) -> Vector2<T> {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl <T: BaseNum> Sub<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn sub(self, other: Vector2<T>) -> Point2<T> {
        Point2::new(self.x - other.x, self.y - other.y)
    }
}

impl <T: BaseNum> SubAssign<Vector2<T>> for Point2<T> {
    fn sub_assign(&mut self, other: Vector2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl <T: BaseNum> Mul<T> for Point2<T> {
    type Output = Point2<T>;

    fn mul(self, scalar: T) -> Point2<T> {
        Point2::new(self.x * scalar, self.y * scalar)
    }
}

impl <T: BaseNum> MulAssign<T> for Point2<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl <T: BaseNum> Div<T> for Point2<T> {
    type Output = Point2<T>;

    fn div(self, scalar: T) -> Point2<T> {
        Point2::new(self.x / scalar, self.y / scalar)
    }
}

impl <T: BaseNum> DivAssign<T> for Point2<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl <T: BaseNum> ComponentWise for Point2<T> {
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

    fn min(self, other: Point2<T>) -> Point2<T> {
        Point2::new(partial_min(self.x, other.x), partial_min(self.y, other.y))
    }

    fn max(self, other: Point2<T>) -> Point2<T> {
        Point2::new(partial_max(self.x, other.x), partial_max(self.y, other.y))
    }
}

impl <T: BaseNum + Signed> ComponentWiseSigned for Point2<T> {
    fn abs(self) -> Point2<T> {
        Point2::new(self.x.abs(), self.y.abs())
    }
}

impl <T: BaseFloat> ComponentWiseFloat for Point2<T> {
    fn floor(self) -> Point2<T> {
        Point2::new(self.x.floor(), self.y.floor())
    }

    fn ceil(self) -> Point2<T> {
        Point2::new(self.x.ceil(), self.y.ceil())
    }
}

impl <T: BaseFloat> MetricSpace for Point2<T> {
    type Scalar = T;

    fn distance_squared(self, other: Point2<T>) -> T {
        (self - other).magnitude_squared()
    }
}

impl <T: BaseFloat> LinearInterpolate for Point2<T> {
    type Scalar = T;
}

pub type Point2i = Point2<IntScalar>;
pub type Point2f = Point2<FloatScalar>;
pub type Point3i = Point3<IntScalar>;
pub type Point3f = Point3<FloatScalar>;