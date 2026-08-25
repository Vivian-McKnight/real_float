use crate::Real;
use core::num::FpCategory;
use num_traits::{
    Num,
    bounds::{Bounded, LowerBounded, UpperBounded},
    cast::*,
    float::{Float, FloatConst, FloatCore},
    identities::{ConstOne, ConstZero, One, Zero},
    ops::{euclid, inv, mul_add},
    pow::Pow,
    // real::Real,
    sign::{Signed, Unsigned},
};

impl Zero for Real<f32> {
    fn zero() -> Self {
        Real(0.0)
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl One for Real<f32> {
    fn one() -> Self {
        Real(1.0)
    }
}

impl Num for Real<f32> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl ToPrimitive for Real<f32> {
    fn to_i64(&self) -> Option<i64> {
        ToPrimitive::to_i64(&self.0)
    }

    fn to_u64(&self) -> Option<u64> {
        ToPrimitive::to_u64(&self.0)
    }

    fn to_f32(&self) -> Option<f32> {
        Some(self.0)
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.0 as f64)
    }
}

impl NumCast for Real<f32> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f32().map(|x| Real(x))
    }
}

impl FloatCore for Real<f32> {
    fn classify(self) -> FpCategory {
        FloatCore::classify(self.0)
    }

    fn epsilon() -> Self {
        Real(<f32 as FloatCore>::epsilon())
    }

    fn infinity() -> Self {
        Real(<f32 as FloatCore>::infinity())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        FloatCore::integer_decode(self.0)
    }

    fn max_value() -> Self {
        Real(<f32 as FloatCore>::max_value())
    }

    fn min_positive_value() -> Self {
        Real(<f32 as FloatCore>::min_positive_value())
    }

    fn min_value() -> Self {
        Real(<f32 as FloatCore>::min_value())
    }

    fn nan() -> Self {
        Real(<f32 as FloatCore>::nan())
    }

    fn neg_infinity() -> Self {
        Real(<f32 as FloatCore>::neg_infinity())
    }

    fn neg_zero() -> Self {
        Real(<f32 as FloatCore>::neg_zero())
    }

    fn to_degrees(self) -> Self {
        Real(FloatCore::to_degrees(self.0))
    }

    fn to_radians(self) -> Self {
        Real(FloatCore::to_radians(self.0))
    }
}

impl Float for Real<f32> {
    fn abs(self) -> Self {
        FloatCore::abs(self)
    }

    fn acos(self) -> Self {
        self.acos()
    }

    fn acosh(self) -> Self {
        self.acosh()
    }

    fn asin(self) -> Self {
        self.asin()
    }

    fn asinh(self) -> Self {
        self.asinh()
    }

    fn atan(self) -> Self {
        self.atan()
    }

    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }

    fn atanh(self) -> Self {
        self.atanh()
    }

    fn cbrt(self) -> Self {
        self.cbrt()
    }

    fn ceil(self) -> Self {
        FloatCore::ceil(self)
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        FloatCore::clamp(self, min, max)
    }

    fn classify(self) -> core::num::FpCategory {
        FloatCore::classify(self)
    }

    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn cosh(self) -> Self {
        self.cosh()
    }

    fn exp(self) -> Self {
        self.exp()
    }

    fn exp2(self) -> Self {
        self.exp2()
    }

    fn exp_m1(self) -> Self {
        self.exp_m1()
    }

    fn floor(self) -> Self {
        FloatCore::floor(self)
    }

    fn fract(self) -> Self {
        FloatCore::fract(self)
    }

    fn hypot(self, other: Self) -> Self {
        self.hypot(other)
    }

    fn is_finite(self) -> bool {
        FloatCore::is_finite(self)
    }

    fn is_infinite(self) -> bool {
        FloatCore::is_infinite(self)
    }

    fn is_nan(self) -> bool {
        FloatCore::is_nan(self)
    }

    fn is_normal(self) -> bool {
        FloatCore::is_normal(self)
    }

    fn is_sign_negative(self) -> bool {
        FloatCore::is_sign_negative(self)
    }

    fn is_sign_positive(self) -> bool {
        FloatCore::is_sign_positive(self)
    }

    fn is_subnormal(self) -> bool {
        FloatCore::is_subnormal(self)
    }

    fn ln(self) -> Self {
        self.ln()
    }

    fn ln_1p(self) -> Self {
        self.ln_1p()
    }

    fn log(self, base: Self) -> Self {
        self.log(base)
    }

    fn log2(self) -> Self {
        self.log2()
    }

    fn log10(self) -> Self {
        self.log10()
    }

    fn max(self, other: Self) -> Self {
        FloatCore::max(self, other)
    }

    fn min(self, other: Self) -> Self {
        FloatCore::min(self, other)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Real(Float::mul_add(self.0, a.0, b.0))
    }

    fn powf(self, n: Self) -> Self {
        Real(Float::powf(self.0, n.0))
    }

    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }

    fn recip(self) -> Self {
        self.recip()
    }

    fn round(self) -> Self {
        Real(FloatCore::round(self.0))
    }

    fn signum(self) -> Self {
        Real(FloatCore::signum(self.0))
    }

    fn sin(self) -> Self {
        Real(Float::sin(self.0))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (s, c) = Float::sin_cos(self.0);
        (Real(s), Real(c))
    }

    fn sinh(self) -> Self {
        self.sinh()
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn tan(self) -> Self {
        self.tan()
    }

    fn tanh(self) -> Self {
        self.tanh()
    }

    fn to_degrees(self) -> Self {
        self.to_degrees()
    }

    fn to_radians(self) -> Self {
        self.to_radians()
    }

    fn trunc(self) -> Self {
        self.trunc()
    }

    fn abs_sub(self, other: Self) -> Self {
        (self - other).abs()
    }

    fn infinity() -> Self {
        Real(<f32 as FloatCore>::infinity())
    }

    fn neg_infinity() -> Self {
        Real(<f32 as FloatCore>::neg_infinity())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        let inner = self.0;
        FloatCore::integer_decode(inner)
    }

    fn max_value() -> Self {
        Real(<f32 as FloatCore>::max_value())
    }

    fn min_positive_value() -> Self {
        Real(<f32 as FloatCore>::min_positive_value())
    }

    fn min_value() -> Self {
        Real(<f32 as FloatCore>::min_value())
    }

    fn nan() -> Self {
        Real(<f32 as FloatCore>::nan())
    }

    fn neg_zero() -> Self {
        Real(<f32 as FloatCore>::neg_zero())
    }
}
