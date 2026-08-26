#![no_std]

#[cfg(feature = "num")]
pub mod num;

use core::{
    cmp::Ordering,
    iter::{Product, Sum},
    num::FpCategory,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};
use duplicate::{duplicate, duplicate_item};

pub type rf32 = Real<f32>;
pub type rf64 = Real<f64>;

/// Wrapper type over floating point numbers that treats them like real numbers. This allows
/// the compiler to exploit algebraic properties of the real numbers like associativity etc.
/// This can result in shallower dependency depth in the output assembly code, or in the case
/// of loops can help the compiler to perform SIMD vectorisation.
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Real<T>(pub T);

#[duplicate_item(
    imp_type to_bits_type byte_length;
    [f32]    [u32]        [4];
    [f64]    [u64]        [8];
)]
impl Real<imp_type> {
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn abs(self) -> Self {
        Real(self.0.abs())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn acos(self) -> Self {
        Real(self.0.acos())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn acosh(self) -> Self {
        Real(self.0.acosh())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn asin(self) -> Self {
        Real(self.0.asin())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn asinh(self) -> Self {
        Real(self.0.asinh())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn atan(self) -> Self {
        Real(self.0.atan())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn atan2(self, other: Self) -> Self {
        Real(self.0.atan2(other.0))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn atanh(self) -> Self {
        Real(self.0.atanh())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn cbrt(self) -> Self {
        Real(self.0.cbrt())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn ceil(self) -> Self {
        Real(self.0.ceil())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        Real(self.0.clamp(min.0, max.0))
    }

    #[must_use]
    pub const fn classify(self) -> FpCategory {
        self.0.classify()
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn copysign(self, sign: Self) -> Self {
        Real(self.0.copysign(sign.0))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn cos(self) -> Self {
        Real(self.0.cos())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn cosh(self) -> Self {
        Real(self.0.cosh())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn div_euclid(self, rhs: Self) -> Self {
        Real(self.0.div_euclid(rhs.0))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn exp(self) -> Self {
        Real(self.0.exp())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn exp2(self) -> Self {
        Real(self.0.exp2())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn exp_m1(self) -> Self {
        Real(self.0.exp_m1())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn floor(self) -> Self {
        Real(self.0.floor())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn fract(self) -> Self {
        Real(self.0.fract())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn hypot(self, other: Self) -> Self {
        Real(self.0.hypot(other.0))
    }

    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    #[must_use]
    pub const fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    #[must_use]
    pub const fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    #[must_use]
    pub const fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    #[must_use]
    pub const fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    #[must_use]
    pub const fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    #[must_use]
    pub const fn is_subnormal(self) -> bool {
        self.0.is_subnormal()
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn ln(self) -> Self {
        Real(self.0.ln())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn ln_1p(self) -> Self {
        Real(self.0.ln_1p())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log(self, base: Self) -> Self {
        Real(self.0.log(base.0))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log2(self) -> Self {
        Real(self.0.log2())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn log10(self) -> Self {
        Real(self.0.log10())
    }

    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub const fn max(self, other: Self) -> Self {
        Real(self.0.max(other.0))
    }

    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub const fn midpoint(self, other: Self) -> Self {
        Real(self.0.midpoint(other.0))
    }

    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub const fn min(self, other: Self) -> Self {
        Real(self.0.min(other.0))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn mul_add(self, a: Self, b: Self) -> Self {
        Real(self.0.mul_add(a.0, b.0))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn next_down(self) -> Self {
        Real(self.0.next_down())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn next_up(self) -> Self {
        Real(self.0.next_up())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn powf(self, n: Self) -> Self {
        Real(self.0.powf(n.0))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn powi(self, n: i32) -> Self {
        Real(self.0.powi(n))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn recip(self) -> Self {
        Real(self.0.recip())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        Real(self.0.rem_euclid(rhs.0))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn round(self) -> Self {
        Real(self.0.round())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn round_ties_even(self) -> Self {
        Real(self.0.round_ties_even())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn signum(self) -> Self {
        Real(self.0.signum())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sin(self) -> Self {
        Real(self.0.sin())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.0.sin_cos();
        (Real(s), Real(c))
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sinh(self) -> Self {
        Real(self.0.sinh())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn sqrt(self) -> Self {
        Real(self.0.sqrt())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn tan(self) -> Self {
        Real(self.0.tan())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn tanh(self) -> Self {
        Real(self.0.tanh())
    }

    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub const fn to_be_bytes(self) -> [u8; byte_length] {
        self.0.to_be_bytes()
    }

    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub const fn to_bits(self) -> to_bits_type {
        self.0.to_bits()
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn to_degrees(self) -> Self {
        Real(self.0.to_degrees())
    }

    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub const fn to_le_bytes(self) -> [u8; byte_length] {
        self.0.to_le_bytes()
    }

    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub const fn to_ne_bytes(self) -> [u8; byte_length] {
        self.0.to_ne_bytes()
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn to_radians(self) -> Self {
        Real(self.0.to_radians())
    }

    #[must_use]
    pub fn total_cmp(self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn trunc(self) -> Self {
        Real(self.0.trunc())
    }
}

duplicate! {
    [imp_type; [f32]; [f64]]
    impl Add<Real<imp_type>> for Real<imp_type> {
        type Output = Real<imp_type>;

        fn add(self, rhs: Real<imp_type>) -> Self::Output {
            Real(self.0.algebraic_add(rhs.0))
        }
    }

    impl Add<&Real<imp_type>> for Real<imp_type> {
        type Output = <Real<imp_type> as Add>::Output;

        fn add(self, rhs: &Real<imp_type>) -> Self::Output {
            self + *rhs
        }
    }

    impl Add<Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Add>::Output;

        fn add(self, rhs: Real<imp_type>) -> Self::Output {
            *self + rhs
        }
    }

    impl Add<&Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Add>::Output;

        fn add(self, rhs: &Real<imp_type>) -> Self::Output {
            *self + *rhs
        }
    }

    impl AddAssign<Real<imp_type>> for Real<imp_type> {
        fn add_assign(&mut self, rhs: Real<imp_type>) {
            *self = *self + rhs
        }
    }

    impl AddAssign<&Real<imp_type>> for Real<imp_type> {
        fn add_assign(&mut self, rhs: &Real<imp_type>) {
            *self = *self + *rhs
        }
    }

    impl Sub<Real<imp_type>> for Real<imp_type> {
        type Output = Real<imp_type>;

        fn sub(self, rhs: Real<imp_type>) -> Self::Output {
            Real(self.0.algebraic_sub(rhs.0))
        }
    }

    impl Sub<&Real<imp_type>> for Real<imp_type> {
        type Output = <Real<imp_type> as Sub>::Output;

        fn sub(self, rhs: &Real<imp_type>) -> Self::Output {
            self + *rhs
        }
    }

    impl Sub<Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Sub>::Output;

        fn sub(self, rhs: Real<imp_type>) -> Self::Output {
            *self + rhs
        }
    }

    impl Sub<&Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Sub>::Output;

        fn sub(self, rhs: &Real<imp_type>) -> Self::Output {
            *self + *rhs
        }
    }

    impl SubAssign<Real<imp_type>> for Real<imp_type> {
        fn sub_assign(&mut self, rhs: Real<imp_type>) {
            *self = *self - rhs
        }
    }

    impl SubAssign<&Real<imp_type>> for Real<imp_type> {
        fn sub_assign(&mut self, rhs: &Real<imp_type>) {
            *self = *self - *rhs
        }
    }

    impl Mul<Real<imp_type>> for Real<imp_type> {
        type Output = Real<imp_type>;

        fn mul(self, rhs: Real<imp_type>) -> Self::Output {
            Real(self.0.algebraic_mul(rhs.0))
        }
    }

    impl Mul<&Real<imp_type>> for Real<imp_type> {
        type Output = <Real<imp_type> as Mul>::Output;

        fn mul(self, rhs: &Real<imp_type>) -> Self::Output {
            self + *rhs
        }
    }

    impl Mul<Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Mul>::Output;

        fn mul(self, rhs: Real<imp_type>) -> Self::Output {
            *self + rhs
        }
    }

    impl Mul<&Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Mul>::Output;

        fn mul(self, rhs: &Real<imp_type>) -> Self::Output {
            *self + *rhs
        }
    }

    impl MulAssign<Real<imp_type>> for Real<imp_type> {
        fn mul_assign(&mut self, rhs: Real<imp_type>) {
            *self = *self * rhs
        }
    }

    impl MulAssign<&Real<imp_type>> for Real<imp_type> {
        fn mul_assign(&mut self, rhs: &Real<imp_type>) {
            *self = *self * *rhs
        }
    }

    impl Div<Real<imp_type>> for Real<imp_type> {
        type Output = Real<imp_type>;

        fn div(self, rhs: Real<imp_type>) -> Self::Output {
            Real(self.0.algebraic_div(rhs.0))
        }
    }

    impl Div<&Real<imp_type>> for Real<imp_type> {
        type Output = <Real<imp_type> as Div>::Output;

        fn div(self, rhs: &Real<imp_type>) -> Self::Output {
            self + *rhs
        }
    }

    impl Div<Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Div>::Output;

        fn div(self, rhs: Real<imp_type>) -> Self::Output {
            *self + rhs
        }
    }

    impl Div<&Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Div>::Output;

        fn div(self, rhs: &Real<imp_type>) -> Self::Output {
            *self + *rhs
        }
    }

    impl DivAssign<Real<imp_type>> for Real<imp_type> {
        fn div_assign(&mut self, rhs: Real<imp_type>) {
            *self = *self / rhs
        }
    }

    impl DivAssign<&Real<imp_type>> for Real<imp_type> {
        fn div_assign(&mut self, rhs: &Real<imp_type>) {
            *self = *self / *rhs
        }
    }

    impl Rem<Real<imp_type>> for Real<imp_type> {
        type Output = Real<imp_type>;

        fn rem(self, rhs: Real<imp_type>) -> Self::Output {
            Real(self.0.algebraic_rem(rhs.0))
        }
    }

    impl Rem<&Real<imp_type>> for Real<imp_type> {
        type Output = <Real<imp_type> as Rem>::Output;

        fn rem(self, rhs: &Real<imp_type>) -> Self::Output {
            self + *rhs
        }
    }

    impl Rem<Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Rem>::Output;

        fn rem(self, rhs: Real<imp_type>) -> Self::Output {
            *self + rhs
        }
    }

    impl Rem<&Real<imp_type>> for &Real<imp_type> {
        type Output = <Real<imp_type> as Rem>::Output;

        fn rem(self, rhs: &Real<imp_type>) -> Self::Output {
            *self + *rhs
        }
    }

    impl RemAssign<Real<imp_type>> for Real<imp_type> {
        fn rem_assign(&mut self, rhs: Real<imp_type>) {
            *self = *self % rhs
        }
    }

    impl RemAssign<&Real<imp_type>> for Real<imp_type> {
        fn rem_assign(&mut self, rhs: &Real<imp_type>) {
            *self = *self % *rhs
        }
    }

    impl Neg for Real<imp_type> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Real(self.0.neg())
        }
    }

    impl Sum<Real<imp_type>> for Real<imp_type> {
        fn sum<I>(iter: I) -> Real<imp_type>
        where
            I: Iterator<Item = Real<imp_type>>,
        {
            iter.fold(Real(0.0), |acc, x| acc + x)
        }
    }

    impl<'a> Sum<&'a Real<imp_type>> for Real<imp_type> {
        fn sum<I>(iter: I) -> Real<imp_type>
        where
            I: Iterator<Item = &'a Real<imp_type>>,
        {
            iter.fold(Real(0.0), |acc, x| acc + x)
        }
    }

    impl Product<Real<imp_type>> for Real<imp_type> {
        fn product<I>(iter: I) -> Real<imp_type>
        where
            I: Iterator<Item = Real<imp_type>>,
        {
            iter.fold(Real(1.0), |acc, x| acc * x)
        }
    }

    impl<'a> Product<&'a Real<imp_type>> for Real<imp_type> {
        fn product<I>(iter: I) -> Real<imp_type>
        where
            I: Iterator<Item = &'a Real<imp_type>>,
        {
            iter.fold(Real(1.0), |acc, x| acc * x)
        }
    }
}
