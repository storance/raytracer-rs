use std::cmp;
use std::fmt;
use std::ops;
use num;

use math::common::LinearInterpolate;

pub type IntScalar = i32;
#[cfg(not(feature = "float64"))]
pub type FloatScalar = f32;
#[cfg(feature = "float64")]
pub type FloatScalar = f64;

pub trait BaseNum where
    Self: Copy + Clone + fmt::Debug + cmp::PartialOrd,
    Self: num::Num + num::NumCast + num::ToPrimitive,
    Self: ops::AddAssign + ops::SubAssign + ops::MulAssign + ops::DivAssign {}

pub trait BaseFloat: BaseNum + num::Float + num::Signed {}

pub trait BaseInt: BaseNum {}

impl BaseNum for i8 {}
impl BaseNum for i16 {}
impl BaseNum for i32 {}
impl BaseNum for i64 {}
impl BaseNum for isize {}
impl BaseNum for u8 {}
impl BaseNum for u16 {}
impl BaseNum for u32 {}
impl BaseNum for u64 {}
impl BaseNum for usize {}

impl BaseNum for f32 {}
impl BaseNum for f64 {}

impl BaseInt for i8 {}
impl BaseInt for i16 {}
impl BaseInt for i32 {}
impl BaseInt for i64 {}
impl BaseInt for isize {}
impl BaseInt for u8 {}
impl BaseInt for u16 {}
impl BaseInt for u32 {}
impl BaseInt for u64 {}
impl BaseInt for usize {}


impl BaseFloat for f32 {}
impl BaseFloat for f64 {}

pub fn partial_min<T: cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn partial_max<T: cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

impl LinearInterpolate for f32 {
    type Scalar = f32;
}

impl LinearInterpolate for f64 {
    type Scalar = f64;
}