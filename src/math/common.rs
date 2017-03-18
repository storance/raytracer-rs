use num::{Zero, One, Signed, Float};
use math::scalar::{BaseNum, BaseFloat};
use std::ops::{Add, Sub, Mul, Div, Index, Neg};

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

pub trait InnerProduct<RHS = Self>: VectorSpace {
    fn dot(self, other: RHS) -> Self::Scalar;
}

pub trait CrossProduct<RHS = Self>: VectorSpace {
    type CrossOutput: VectorSpace;

    fn cross(self, other: RHS) -> Self::CrossOutput;
}

pub trait InnerProductSpace: InnerProduct where
    <Self as VectorSpace>::Scalar: BaseFloat, {
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

pub trait MetricSpace<RHS = Self>: Copy + Clone {
    type Scalar: BaseFloat;

    fn distance(self, other: RHS) -> Self::Scalar {
        Float::sqrt(self.distance_squared(other))
    }

    fn distance_squared(self, other: RHS) -> Self::Scalar;
}

pub trait LinearInterpolate: Copy + Clone where
    Self: Add<Self, Output = Self>,
    Self: Mul<<Self as LinearInterpolate>::Scalar, Output = Self>, {
    type Scalar: BaseFloat;

    fn lerp(self, other: Self, t: Self::Scalar) -> Self {
        self * (Self::Scalar::one() - t) + other * t
    }
}

pub fn dot<T: InnerProduct, U: InnerProduct<T>>(v1: U, v2: T) -> U::Scalar {
    v1.dot(v2)
}

pub fn abs_dot<T: InnerProduct, U: InnerProduct<T>>(v1: U, v2: T) -> U::Scalar 
    where U::Scalar: Signed {
    v1.dot(v2).abs()
}

pub fn cross<T: CrossProduct, U: CrossProduct<T>>(v1: U, v2: T) -> U::CrossOutput {
    v1.cross(v2)
}

pub fn min_component<T: ComponentWise>(v: T) -> T::Scalar {
    v.min_component()
}

pub fn max_component<T: ComponentWise>(v: T) -> T::Scalar {
    v.max_component()
}

pub fn distance<T: MetricSpace>(v1: T, v2: T) -> T::Scalar {
    v1.distance(v2)
}

pub fn distance_squared<T: MetricSpace>(v1: T, v2: T) -> T::Scalar {
    v1.distance_squared(v2)
}

pub fn component_wise_min<T: ComponentWise>(v1: T, v2: T) -> T {
    v1.min(v2)
}

pub fn component_wise_max<T: ComponentWise>(v1: T, v2: T) -> T {
    v1.max(v2)
}

pub fn face_forward<T: InnerProduct, U: InnerProduct<T> + Neg<Output = U>>(v1: U, v2: T) -> U {
    if dot(v1, v2) < U::Scalar::zero() {
        -v1
    } else {
        v1
    }
}