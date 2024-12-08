//! Numerical Traits - For more convenient trait bounding
use num::{Bounded, CheckedMul, FromPrimitive, Num, NumCast, ToPrimitive};
use rand::distributions::uniform::SampleUniform;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use num_traits::{real::Real};

pub trait Number:
    Num
    + NumCast
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + ToPrimitive
    + FromPrimitive
    + SampleUniform
    + Send
    + Sync
    + Copy
{
}

impl<T> Number for T where
    T: Num
        + NumCast
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + ToPrimitive
        + FromPrimitive
        + SampleUniform
        + Send
        + Sync
        + Copy
{
}

pub trait Integer: Number + Bounded + Ord + CheckedMul {}

impl<T> Integer for T where T: Number + Bounded + Ord + CheckedMul {}

pub trait RealNumber: Number + PartialOrd + Real {}

impl<T> RealNumber for T where T: Number + PartialOrd + Real {}
