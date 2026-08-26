//! num traits support.

use crate::Real;
use core::num::FpCategory;
use duplicate::duplicate;
use num_traits::{
    Num,
    bounds::Bounded,
    cast::*,
    float::{Float, FloatConst, FloatCore},
    identities::{ConstOne, ConstZero, One, Zero},
    ops::{euclid::Euclid, inv::Inv, mul_add::MulAdd},
    pow::Pow,
    sign::Signed,
};

impl NumCast for Real<f32> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f32().map(|x| Real(x))
    }
}

impl NumCast for Real<f64> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_f64().map(|x| Real(x))
    }
}

duplicate! {
    [imp_type; [f32]; [f64]]
    impl Signed for Real<imp_type> {
        fn abs(&self) -> Self {
            Float::abs(*self)
        }

        fn abs_sub(&self, other: &Self) -> Self {
            Float::abs_sub(*self, *other)
        }

        fn signum(&self) -> Self {
            Float::signum(*self)
        }

        fn is_positive(&self) -> bool {
            Float::is_sign_positive(*self)
        }

        fn is_negative(&self) -> bool {
            Float::is_sign_negative(*self)
        }
    }

    impl Bounded for Real<imp_type> {
        fn max_value() -> Self {
            Real(Bounded::max_value())
        }

        fn min_value() -> Self {
            Real(Bounded::min_value())
        }
    }

    impl ConstZero for Real<imp_type> {
        const ZERO: Self = Real(imp_type::ZERO);
    }

    impl Zero for Real<imp_type> {
        fn zero() -> Self {
            ConstZero::ZERO
        }

        fn is_zero(&self) -> bool {
            *self == Self::zero()
        }
    }

    impl ConstOne for Real<imp_type> {
        const ONE: Self = Real(imp_type::ONE);
    }

    impl One for Real<imp_type> {
        fn one() -> Self {
            ConstOne::ONE
        }
    }

    impl FloatConst for Real<imp_type> {
        fn E() -> Self {
            Real(imp_type::E())
        }

        fn FRAC_1_PI() -> Self {
            Real(imp_type::FRAC_1_PI())
        }

        fn FRAC_1_SQRT_2() -> Self {
            Real(imp_type::FRAC_1_SQRT_2())
        }

        fn FRAC_2_PI() -> Self {
            Real(imp_type::FRAC_2_PI())
        }

        fn FRAC_2_SQRT_PI() -> Self {
            Real(imp_type::FRAC_2_SQRT_PI())
        }

        fn FRAC_PI_2() -> Self {
            Real(imp_type::FRAC_PI_2())
        }

        fn FRAC_PI_3() -> Self {
            Real(imp_type::FRAC_PI_3())
        }

        fn FRAC_PI_4() -> Self {
            Real(imp_type::FRAC_PI_4())
        }

        fn FRAC_PI_6() -> Self {
            Real(imp_type::FRAC_PI_6())
        }

        fn FRAC_PI_8() -> Self {
            Real(imp_type::FRAC_PI_8())
        }

        fn LN_10() -> Self {
            Real(imp_type::LN_10())
        }

        fn LN_2() -> Self {
            Real(imp_type::LN_2())
        }

        fn LOG10_2() -> Self
        where
            Self: Sized + core::ops::Div<Self, Output = Self>,
        {
            Real(imp_type::LOG10_2())
        }

        fn LOG10_E() -> Self {
            Real(imp_type::LOG10_E())
        }

        fn LOG2_10() -> Self
        where
            Self: Sized + core::ops::Div<Self, Output = Self>,
        {
            Real(imp_type::LOG2_10())
        }

        fn LOG2_E() -> Self {
            Real(imp_type::LOG2_E())
        }

        fn PI() -> Self {
            Real(imp_type::PI())
        }

        fn SQRT_2() -> Self {
            Real(imp_type::SQRT_2())
        }

        fn TAU() -> Self
        where
            Self: Sized + core::ops::Add<Self, Output = Self>,
        {
            Real(imp_type::TAU())
        }
    }

    impl Num for Real<imp_type> {
        type FromStrRadixErr = num_traits::ParseFloatError;

        fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
            imp_type::from_str_radix(str, radix).map(|x| Real(x))
        }
    }

    impl ToPrimitive for Real<imp_type> {
        fn to_i64(&self) -> Option<i64> {
            ToPrimitive::to_i64(&self.0)
        }

        fn to_u64(&self) -> Option<u64> {
            ToPrimitive::to_u64(&self.0)
        }

        fn to_f32(&self) -> Option<f32> {
            Some(self.0 as f32)
        }

        fn to_f64(&self) -> Option<f64> {
            Some(self.0 as f64)
        }
    }

    impl FloatCore for Real<imp_type> {
        fn classify(self) -> FpCategory {
            FloatCore::classify(self.0)
        }

        fn epsilon() -> Self {
            Real(FloatCore::epsilon())
        }

        fn infinity() -> Self {
            Real(FloatCore::infinity())
        }

        fn integer_decode(self) -> (u64, i16, i8) {
            FloatCore::integer_decode(self.0)
        }

        fn max_value() -> Self {
            Real(FloatCore::max_value())
        }

        fn min_positive_value() -> Self {
            Real(FloatCore::min_positive_value())
        }

        fn min_value() -> Self {
            Real(FloatCore::min_value())
        }

        fn nan() -> Self {
            Real(FloatCore::nan())
        }

        fn neg_infinity() -> Self {
            Real(FloatCore::neg_infinity())
        }

        fn neg_zero() -> Self {
            Real(FloatCore::neg_zero())
        }

        fn to_degrees(self) -> Self {
            Real(FloatCore::to_degrees(self.0))
        }

        fn to_radians(self) -> Self {
            Real(FloatCore::to_radians(self.0))
        }

        fn abs(self) -> Self {
            self.abs()
        }

        fn ceil(self) -> Self {
            self.ceil()
        }

        fn clamp(self, min: Self, max: Self) -> Self {
            self.clamp(min, max)
        }

        fn floor(self) -> Self {
            self.floor()
        }

        fn fract(self) -> Self {
            self.fract()
        }

        fn is_finite(self) -> bool {
            self.is_finite()
        }

        fn is_infinite(self) -> bool {
            self.is_infinite()
        }

        fn is_nan(self) -> bool {
            self.is_nan()
        }

        fn is_normal(self) -> bool {
            self.is_normal()
        }

        fn is_sign_negative(self) -> bool {
            self.is_sign_negative()
        }

        fn is_sign_positive(self) -> bool {
            self.is_sign_positive()
        }

        fn is_subnormal(self) -> bool {
            self.is_subnormal()
        }

        fn max(self, other: Self) -> Self {
            self.max(other)
        }

        fn min(self, other: Self) -> Self {
            self.min(other)
        }

        fn powi(self, exp: i32) -> Self {
            self.powi(exp)
        }

        fn recip(self) -> Self {
            self.recip()
        }

        fn round(self) -> Self {
            self.round()
        }

        fn signum(self) -> Self {
            self.signum()
        }

        fn trunc(self) -> Self {
            self.trunc()
        }
    }

    impl Float for Real<imp_type> {
        fn epsilon() -> Self {
            FloatCore::epsilon()
        }

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
            FloatCore::powi(self, n)
        }

        fn recip(self) -> Self {
            FloatCore::recip(self)
        }

        fn round(self) -> Self {
            FloatCore::round(self)
        }

        fn signum(self) -> Self {
            FloatCore::signum(self)
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
            FloatCore::trunc(self)
        }

        fn abs_sub(self, other: Self) -> Self {
            (self - other).abs()
        }

        fn infinity() -> Self {
            FloatCore::infinity()
        }

        fn neg_infinity() -> Self {
            FloatCore::neg_infinity()
        }

        fn integer_decode(self) -> (u64, i16, i8) {
            let inner = self.0;
            FloatCore::integer_decode(inner)
        }

        fn max_value() -> Self {
            FloatCore::max_value()
        }

        fn min_positive_value() -> Self {
            FloatCore::min_positive_value()
        }

        fn min_value() -> Self {
            FloatCore::min_value()
        }

        fn nan() -> Self {
            FloatCore::nan()
        }

        fn neg_zero() -> Self {
            FloatCore::neg_zero()
        }
    }

    impl Euclid for Real<imp_type> {
        fn div_euclid(&self, v: &Self) -> Self {
            Real::<imp_type>::div_euclid(*self, *v)
        }

        fn rem_euclid(&self, v: &Self) -> Self {
            Real::<imp_type>::rem_euclid(*self, *v)
        }
    }

    impl Inv for Real<imp_type> {
        type Output = Self;

        fn inv(self) -> Self::Output {
            Real(imp_type::inv(self.0))
        }
    }

    impl MulAdd for Real<imp_type> {
        type Output = Self;

        fn mul_add(self, a: Self, b: Self) -> Self::Output {
            self.mul_add(a, b)
        }
    }

    impl Pow<Self> for Real<imp_type> {
        type Output = Self;

        fn pow(self, rhs: Self) -> Self::Output {
            self.powf(rhs)
        }
    }

}
