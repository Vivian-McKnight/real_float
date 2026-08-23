use core::{
    cmp::Ordering,
    iter::{Product, Sum},
    num::FpCategory,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

/// Wrapper type over floating point numbers that treats them like real numbers. This allows
/// the compiler to exploit algebraic properties of the real numbers like associativity etc.
/// This can result in shallower dependency depth in the output assembly code, or in the case
/// of loops can help the compiler to perform SIMD vectorisation.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Real<T>(pub T);

// f32
impl Real<f32> {
    pub fn abs(self) -> Self {
        Real(self.0.abs())
    }

    pub fn acos(self) -> Self {
        Real(self.0.acos())
    }

    pub fn acosh(self) -> Self {
        Real(self.0.acosh())
    }

    pub fn asin(self) -> Self {
        Real(self.0.asin())
    }

    pub fn asinh(self) -> Self {
        Real(self.0.asinh())
    }

    pub fn atan(self) -> Self {
        Real(self.0.atan())
    }

    pub fn atan2(self, other: Self) -> Self {
        Real(self.0.atan2(other.0))
    }

    pub fn atanh(self) -> Self {
        Real(self.0.atanh())
    }

    pub fn cbrt(self) -> Self {
        Real(self.0.cbrt())
    }

    pub fn ceil(self) -> Self {
        Real(self.0.ceil())
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Real(self.0.clamp(min.0, max.0))
    }

    pub fn classify(self) -> FpCategory {
        self.0.classify()
    }

    pub fn copysign(self, sign: Self) -> Self {
        Real(self.0.copysign(sign.0))
    }

    pub fn cos(self) -> Self {
        Real(self.0.cos())
    }

    pub fn cosh(self) -> Self {
        Real(self.0.cosh())
    }

    pub fn div_euclid(self, rhs: Self) -> Self {
        Real(self.0.div_euclid(rhs.0))
    }

    pub fn exp(self) -> Self {
        Real(self.0.exp())
    }

    pub fn exp2(self) -> Self {
        Real(self.0.exp2())
    }

    pub fn exp_m1(self) -> Self {
        Real(self.0.exp_m1())
    }

    pub fn floor(self) -> Self {
        Real(self.0.floor())
    }

    pub fn fract(self) -> Self {
        Real(self.0.fract())
    }

    pub fn hypot(self, other: Self) -> Self {
        Real(self.0.hypot(other.0))
    }

    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    pub fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    pub fn is_subnormal(self) -> bool {
        self.0.is_subnormal()
    }

    pub fn ln(self) -> Self {
        Real(self.0.ln())
    }

    pub fn ln_1p(self) -> Self {
        Real(self.0.ln_1p())
    }

    pub fn log(self, base: Self) -> Self {
        Real(self.0.log(base.0))
    }

    pub fn log2(self) -> Self {
        Real(self.0.log2())
    }

    pub fn log10(self) -> Self {
        Real(self.0.log10())
    }

    pub fn max(self, other: Self) -> Self {
        Real(self.0.max(other.0))
    }

    pub fn midpoint(self, other: Self) -> Self {
        Real(self.0.midpoint(other.0))
    }

    pub fn min(self, other: Self) -> Self {
        Real(self.0.min(other.0))
    }

    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Real(self.0.mul_add(a.0, b.0))
    }

    pub fn next_down(self) -> Self {
        Real(self.0.next_down())
    }

    pub fn next_up(self) -> Self {
        Real(self.0.next_up())
    }

    pub fn powf(self, n: Self) -> Self {
        Real(self.0.powf(n.0))
    }

    pub fn powi(self, n: i32) -> Self {
        Real(self.0.powi(n))
    }

    pub fn recip(self) -> Self {
        Real(self.0.recip())
    }

    pub fn rem_euclid(self, rhs: Self) -> Self {
        Real(self.0.rem_euclid(rhs.0))
    }

    pub fn round(self) -> Self {
        Real(self.0.round())
    }

    pub fn round_ties_even(self) -> Self {
        Real(self.0.round_ties_even())
    }

    pub fn signum(self) -> Self {
        Real(self.0.signum())
    }

    pub fn sin(self) -> Self {
        Real(self.0.sin())
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.0.sin_cos();
        (Real(s), Real(c))
    }

    pub fn sinh(self) -> Self {
        Real(self.0.sinh())
    }

    pub fn sqrt(self) -> Self {
        Real(self.0.sqrt())
    }

    pub fn tan(self) -> Self {
        Real(self.0.tan())
    }

    pub fn tanh(self) -> Self {
        Real(self.0.tanh())
    }

    pub fn to_be_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    pub fn to_bits(self) -> u32 {
        self.0.to_bits()
    }

    pub fn to_degrees(self) -> Self {
        Real(self.0.to_degrees())
    }

    pub fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }

    pub fn to_ne_bytes(self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }

    pub fn to_radians(self) -> Self {
        Real(self.0.to_radians())
    }

    pub fn total_cmp(self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }

    pub fn trunc(self) -> Self {
        Real(self.0.trunc())
    }
}

impl Add<Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn add(self, rhs: Real<f32>) -> Self::Output {
        Real(self.0.algebraic_add(rhs.0))
    }
}

impl Add<&Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn add(self, rhs: &Real<f32>) -> Self::Output {
        self + *rhs
    }
}

impl Add<Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn add(self, rhs: Real<f32>) -> Self::Output {
        *self + rhs
    }
}

impl Add<&Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn add(self, rhs: &Real<f32>) -> Self::Output {
        *self + *rhs
    }
}

impl AddAssign<Real<f32>> for Real<f32> {
    fn add_assign(&mut self, rhs: Real<f32>) {
        *self = *self + rhs
    }
}

impl AddAssign<&Real<f32>> for Real<f32> {
    fn add_assign(&mut self, rhs: &Real<f32>) {
        *self = *self + *rhs
    }
}

impl Sub<Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn sub(self, rhs: Real<f32>) -> Self::Output {
        Real(self.0.algebraic_sub(rhs.0))
    }
}

impl Sub<&Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn sub(self, rhs: &Real<f32>) -> Self::Output {
        self + *rhs
    }
}

impl Sub<Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn sub(self, rhs: Real<f32>) -> Self::Output {
        *self + rhs
    }
}

impl Sub<&Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn sub(self, rhs: &Real<f32>) -> Self::Output {
        *self + *rhs
    }
}

impl SubAssign<Real<f32>> for Real<f32> {
    fn sub_assign(&mut self, rhs: Real<f32>) {
        *self = *self - rhs
    }
}

impl SubAssign<&Real<f32>> for Real<f32> {
    fn sub_assign(&mut self, rhs: &Real<f32>) {
        *self = *self - *rhs
    }
}

impl Mul<Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn mul(self, rhs: Real<f32>) -> Self::Output {
        Real(self.0.algebraic_mul(rhs.0))
    }
}

impl Mul<&Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn mul(self, rhs: &Real<f32>) -> Self::Output {
        self + *rhs
    }
}

impl Mul<Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn mul(self, rhs: Real<f32>) -> Self::Output {
        *self + rhs
    }
}

impl Mul<&Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn mul(self, rhs: &Real<f32>) -> Self::Output {
        *self + *rhs
    }
}

impl MulAssign<Real<f32>> for Real<f32> {
    fn mul_assign(&mut self, rhs: Real<f32>) {
        *self = *self * rhs
    }
}

impl MulAssign<&Real<f32>> for Real<f32> {
    fn mul_assign(&mut self, rhs: &Real<f32>) {
        *self = *self * *rhs
    }
}

impl Div<Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn div(self, rhs: Real<f32>) -> Self::Output {
        Real(self.0.algebraic_div(rhs.0))
    }
}

impl Div<&Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn div(self, rhs: &Real<f32>) -> Self::Output {
        self + *rhs
    }
}

impl Div<Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn div(self, rhs: Real<f32>) -> Self::Output {
        *self + rhs
    }
}

impl Div<&Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn div(self, rhs: &Real<f32>) -> Self::Output {
        *self + *rhs
    }
}

impl DivAssign<Real<f32>> for Real<f32> {
    fn div_assign(&mut self, rhs: Real<f32>) {
        *self = *self / rhs
    }
}

impl DivAssign<&Real<f32>> for Real<f32> {
    fn div_assign(&mut self, rhs: &Real<f32>) {
        *self = *self / *rhs
    }
}

impl Rem<Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn rem(self, rhs: Real<f32>) -> Self::Output {
        Real(self.0.algebraic_rem(rhs.0))
    }
}

impl Rem<&Real<f32>> for Real<f32> {
    type Output = Real<f32>;

    fn rem(self, rhs: &Real<f32>) -> Self::Output {
        self + *rhs
    }
}

impl Rem<Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn rem(self, rhs: Real<f32>) -> Self::Output {
        *self + rhs
    }
}

impl Rem<&Real<f32>> for &Real<f32> {
    type Output = Real<f32>;

    fn rem(self, rhs: &Real<f32>) -> Self::Output {
        *self + *rhs
    }
}

impl RemAssign<Real<f32>> for Real<f32> {
    fn rem_assign(&mut self, rhs: Real<f32>) {
        *self = *self % rhs
    }
}

impl RemAssign<&Real<f32>> for Real<f32> {
    fn rem_assign(&mut self, rhs: &Real<f32>) {
        *self = *self % *rhs
    }
}

impl Neg for Real<f32> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Real(self.0.neg())
    }
}

impl Sum<Real<f32>> for Real<f32> {
    fn sum<I: Iterator<Item = Real<f32>>>(iter: I) -> Real<f32> {
        iter.fold(Real(0f32), |acc, x| acc + x)
    }
}

impl<'a> Sum<&'a Real<f32>> for Real<f32> {
    fn sum<I: Iterator<Item = &'a Real<f32>>>(iter: I) -> Real<f32> {
        iter.fold(Real(0f32), |acc, x| acc + x)
    }
}

impl Product<Real<f32>> for Real<f32> {
    fn product<I: Iterator<Item = Real<f32>>>(iter: I) -> Real<f32> {
        iter.fold(Real(1f32), |acc, x| acc * x)
    }
}

impl<'a> Product<&'a Real<f32>> for Real<f32> {
    fn product<I: Iterator<Item = &'a Real<f32>>>(iter: I) -> Real<f32> {
        iter.fold(Real(1f32), |acc, x| acc * x)
    }
}

// f64
impl Real<f64> {
    pub fn abs(self) -> Self {
        Real(self.0.abs())
    }

    pub fn acos(self) -> Self {
        Real(self.0.acos())
    }

    pub fn acosh(self) -> Self {
        Real(self.0.acosh())
    }

    pub fn asin(self) -> Self {
        Real(self.0.asin())
    }

    pub fn asinh(self) -> Self {
        Real(self.0.asinh())
    }

    pub fn atan(self) -> Self {
        Real(self.0.atan())
    }

    pub fn atan2(self, other: Self) -> Self {
        Real(self.0.atan2(other.0))
    }

    pub fn atanh(self) -> Self {
        Real(self.0.atanh())
    }

    pub fn cbrt(self) -> Self {
        Real(self.0.cbrt())
    }

    pub fn ceil(self) -> Self {
        Real(self.0.ceil())
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Real(self.0.clamp(min.0, max.0))
    }

    pub fn classify(self) -> FpCategory {
        self.0.classify()
    }

    pub fn copysign(self, sign: Self) -> Self {
        Real(self.0.copysign(sign.0))
    }

    pub fn cos(self) -> Self {
        Real(self.0.cos())
    }

    pub fn cosh(self) -> Self {
        Real(self.0.cosh())
    }

    pub fn div_euclid(self, rhs: Self) -> Self {
        Real(self.0.div_euclid(rhs.0))
    }

    pub fn exp(self) -> Self {
        Real(self.0.exp())
    }

    pub fn exp2(self) -> Self {
        Real(self.0.exp2())
    }

    pub fn exp_m1(self) -> Self {
        Real(self.0.exp_m1())
    }

    pub fn floor(self) -> Self {
        Real(self.0.floor())
    }

    pub fn fract(self) -> Self {
        Real(self.0.fract())
    }

    pub fn hypot(self, other: Self) -> Self {
        Real(self.0.hypot(other.0))
    }

    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    pub fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    pub fn is_subnormal(self) -> bool {
        self.0.is_subnormal()
    }

    pub fn ln(self) -> Self {
        Real(self.0.ln())
    }

    pub fn ln_1p(self) -> Self {
        Real(self.0.ln_1p())
    }

    pub fn log(self, base: Self) -> Self {
        Real(self.0.log(base.0))
    }

    pub fn log2(self) -> Self {
        Real(self.0.log2())
    }

    pub fn log10(self) -> Self {
        Real(self.0.log10())
    }

    pub fn max(self, other: Self) -> Self {
        Real(self.0.max(other.0))
    }

    pub fn midpoint(self, other: Self) -> Self {
        Real(self.0.midpoint(other.0))
    }

    pub fn min(self, other: Self) -> Self {
        Real(self.0.min(other.0))
    }

    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Real(self.0.mul_add(a.0, b.0))
    }

    pub fn next_down(self) -> Self {
        Real(self.0.next_down())
    }

    pub fn next_up(self) -> Self {
        Real(self.0.next_up())
    }

    pub fn powf(self, n: Self) -> Self {
        Real(self.0.powf(n.0))
    }

    pub fn powi(self, n: i32) -> Self {
        Real(self.0.powi(n))
    }

    pub fn recip(self) -> Self {
        Real(self.0.recip())
    }

    pub fn rem_euclid(self, rhs: Self) -> Self {
        Real(self.0.rem_euclid(rhs.0))
    }

    pub fn round(self) -> Self {
        Real(self.0.round())
    }

    pub fn round_ties_even(self) -> Self {
        Real(self.0.round_ties_even())
    }

    pub fn signum(self) -> Self {
        Real(self.0.signum())
    }

    pub fn sin(self) -> Self {
        Real(self.0.sin())
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.0.sin_cos();
        (Real(s), Real(c))
    }

    pub fn sinh(self) -> Self {
        Real(self.0.sinh())
    }

    pub fn sqrt(self) -> Self {
        Real(self.0.sqrt())
    }

    pub fn tan(self) -> Self {
        Real(self.0.tan())
    }

    pub fn tanh(self) -> Self {
        Real(self.0.tanh())
    }

    pub fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub fn to_bits(self) -> u64 {
        self.0.to_bits()
    }

    pub fn to_degrees(self) -> Self {
        Real(self.0.to_degrees())
    }

    pub fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    pub fn to_ne_bytes(self) -> [u8; 8] {
        self.0.to_ne_bytes()
    }

    pub fn to_radians(self) -> Self {
        Real(self.0.to_radians())
    }

    pub fn total_cmp(self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }

    pub fn trunc(self) -> Self {
        Real(self.0.trunc())
    }
}

impl Add<Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn add(self, rhs: Real<f64>) -> Self::Output {
        Real(self.0.algebraic_add(rhs.0))
    }
}

impl Add<&Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn add(self, rhs: &Real<f64>) -> Self::Output {
        self + *rhs
    }
}

impl Add<Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn add(self, rhs: Real<f64>) -> Self::Output {
        *self + rhs
    }
}

impl Add<&Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn add(self, rhs: &Real<f64>) -> Self::Output {
        *self + *rhs
    }
}

impl AddAssign<Real<f64>> for Real<f64> {
    fn add_assign(&mut self, rhs: Real<f64>) {
        *self = *self + rhs
    }
}

impl AddAssign<&Real<f64>> for Real<f64> {
    fn add_assign(&mut self, rhs: &Real<f64>) {
        *self = *self + *rhs
    }
}

impl Sub<Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn sub(self, rhs: Real<f64>) -> Self::Output {
        Real(self.0.algebraic_sub(rhs.0))
    }
}

impl Sub<&Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn sub(self, rhs: &Real<f64>) -> Self::Output {
        self + *rhs
    }
}

impl Sub<Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn sub(self, rhs: Real<f64>) -> Self::Output {
        *self + rhs
    }
}

impl Sub<&Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn sub(self, rhs: &Real<f64>) -> Self::Output {
        *self + *rhs
    }
}

impl SubAssign<Real<f64>> for Real<f64> {
    fn sub_assign(&mut self, rhs: Real<f64>) {
        *self = *self - rhs
    }
}

impl SubAssign<&Real<f64>> for Real<f64> {
    fn sub_assign(&mut self, rhs: &Real<f64>) {
        *self = *self - *rhs
    }
}

impl Mul<Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn mul(self, rhs: Real<f64>) -> Self::Output {
        Real(self.0.algebraic_mul(rhs.0))
    }
}

impl Mul<&Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn mul(self, rhs: &Real<f64>) -> Self::Output {
        self + *rhs
    }
}

impl Mul<Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn mul(self, rhs: Real<f64>) -> Self::Output {
        *self + rhs
    }
}

impl Mul<&Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn mul(self, rhs: &Real<f64>) -> Self::Output {
        *self + *rhs
    }
}

impl MulAssign<Real<f64>> for Real<f64> {
    fn mul_assign(&mut self, rhs: Real<f64>) {
        *self = *self * rhs
    }
}

impl MulAssign<&Real<f64>> for Real<f64> {
    fn mul_assign(&mut self, rhs: &Real<f64>) {
        *self = *self * *rhs
    }
}

impl Div<Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn div(self, rhs: Real<f64>) -> Self::Output {
        Real(self.0.algebraic_div(rhs.0))
    }
}

impl Div<&Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn div(self, rhs: &Real<f64>) -> Self::Output {
        self + *rhs
    }
}

impl Div<Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn div(self, rhs: Real<f64>) -> Self::Output {
        *self + rhs
    }
}

impl Div<&Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn div(self, rhs: &Real<f64>) -> Self::Output {
        *self + *rhs
    }
}

impl DivAssign<Real<f64>> for Real<f64> {
    fn div_assign(&mut self, rhs: Real<f64>) {
        *self = *self / rhs
    }
}

impl DivAssign<&Real<f64>> for Real<f64> {
    fn div_assign(&mut self, rhs: &Real<f64>) {
        *self = *self / *rhs
    }
}

impl Rem<Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn rem(self, rhs: Real<f64>) -> Self::Output {
        Real(self.0.algebraic_rem(rhs.0))
    }
}

impl Rem<&Real<f64>> for Real<f64> {
    type Output = Real<f64>;

    fn rem(self, rhs: &Real<f64>) -> Self::Output {
        self + *rhs
    }
}

impl Rem<Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn rem(self, rhs: Real<f64>) -> Self::Output {
        *self + rhs
    }
}

impl Rem<&Real<f64>> for &Real<f64> {
    type Output = Real<f64>;

    fn rem(self, rhs: &Real<f64>) -> Self::Output {
        *self + *rhs
    }
}

impl RemAssign<Real<f64>> for Real<f64> {
    fn rem_assign(&mut self, rhs: Real<f64>) {
        *self = *self % rhs
    }
}

impl RemAssign<&Real<f64>> for Real<f64> {
    fn rem_assign(&mut self, rhs: &Real<f64>) {
        *self = *self % *rhs
    }
}

impl Neg for Real<f64> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Real(self.0.neg())
    }
}

impl Sum<Real<f64>> for Real<f64> {
    fn sum<I: Iterator<Item = Real<f64>>>(iter: I) -> Real<f64> {
        iter.fold(Real(0f64), |acc, x| acc + x)
    }
}

impl<'a> Sum<&'a Real<f64>> for Real<f64> {
    fn sum<I: Iterator<Item = &'a Real<f64>>>(iter: I) -> Real<f64> {
        iter.fold(Real(0f64), |acc, x| acc + x)
    }
}

impl Product<Real<f64>> for Real<f64> {
    fn product<I: Iterator<Item = Real<f64>>>(iter: I) -> Real<f64> {
        iter.fold(Real(1f64), |acc, x| acc * x)
    }
}

impl<'a> Product<&'a Real<f64>> for Real<f64> {
    fn product<I: Iterator<Item = &'a Real<f64>>>(iter: I) -> Real<f64> {
        iter.fold(Real(1f64), |acc, x| acc * x)
    }
}

fn test() {
    let mut x = Real(0f32);
    let y = Real(2f32);
    // x += y;
}
