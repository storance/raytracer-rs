use num::{Zero, One, Signed, Float};
use math::scalar::{BaseNum, BaseFloat};
use std::ops::{Add, Sub, Mul, Div, Index};

#[derive(Debug)]
pub enum Dimension2 {
    X = 1,
    Y = 2,
}

#[derive(Debug)]
pub enum Dimension3 {
    X = 1,
    Y = 2,
    Z = 3,
}

pub trait ComponentWise where 
    Self: Index<usize>,
    Self: Index<<Self as ComponentWise>::Dimension> {
    type Scalar: BaseNum;
    type Dimension;

    fn min_component(self) -> Self::Scalar;

    fn max_component(self) -> Self::Scalar;

    fn max_dimension(self) -> Self::Dimension;

    fn min(self, other: Self) -> Self;

    fn max(self, other: Self) -> Self;
}

pub trait ComponentWiseSigned: ComponentWise where 
    <Self as ComponentWise>::Scalar: BaseNum + Signed {
    fn abs(self) -> Self;
}

pub trait ComponentWiseFloat: ComponentWiseSigned where 
    <Self as ComponentWise>::Scalar: BaseFloat {
    fn floor(self) -> Self;

    fn ceil(self) -> Self;
}

pub trait VectorSpace: Copy + Clone where
    Self: Zero,
    Self: Add<Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: Mul<<Self as VectorSpace>::Scalar, Output = Self>,
    Self: Div<<Self as VectorSpace>::Scalar, Output = Self>, {
    type Scalar: BaseNum;
}

pub trait InnerProductSpace: VectorSpace where
    <Self as VectorSpace>::Scalar: BaseFloat, {
    fn dot(self, other: Self) -> Self::Scalar;

    fn magnitude(self) -> Self::Scalar {
        Float::sqrt(self.magnitude_squared())
    }

    fn magnitude_squared(self) -> Self::Scalar {
        self.dot(self)
    }

    fn normalize(self) -> Self {
        self * (Self::Scalar::one() / self.magnitude())
    }
}

pub trait MetricSpace: Copy + Clone {
    type Scalar: BaseFloat;

    fn distance(self, other: Self) -> Self::Scalar {
        Float::sqrt(self.distance_squared(other))
    }

    fn distance_squared(self, other: Self) -> Self::Scalar;
}

pub trait LinearInterpolate: Copy + Clone where
    Self: Add<Self, Output = Self>,
    Self: Mul<<Self as LinearInterpolate>::Scalar, Output = Self>, {
    type Scalar: BaseFloat;

    fn lerp(self, other: Self, t: Self::Scalar) -> Self {
        self * (Self::Scalar::one() - t) + other * t
    }
}

impl LinearInterpolate for f32 {
    type Scalar = f32;
}

impl LinearInterpolate for f64 {
    type Scalar = f64;
}