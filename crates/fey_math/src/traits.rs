//! Helper traits for scalar and math types.

use std::fmt::{Debug, Display};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

mod private {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
    impl Sealed for usize {}

    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for i128 {}
    impl Sealed for isize {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// A numeric type that can be casted.
///
/// Since all components of this type are the same primitive number type,
/// we can provide methods to cast the entire object to different ones.
pub trait Numeric {
    type AsU8;
    type AsU16;
    type AsU32;
    type AsU64;
    type AsU128;
    type AsUSize;
    type AsI8;
    type AsI16;
    type AsI32;
    type AsI64;
    type AsI128;
    type AsISize;
    type AsF32;
    type AsF64;

    fn to_u8(self) -> Self::AsU8;
    fn to_u16(self) -> Self::AsU16;
    fn to_u32(self) -> Self::AsU32;
    fn to_u64(self) -> Self::AsU64;
    fn to_u128(self) -> Self::AsU128;
    fn to_usize(self) -> Self::AsUSize;

    fn to_i8(self) -> Self::AsI8;
    fn to_i16(self) -> Self::AsI16;
    fn to_i32(self) -> Self::AsI32;
    fn to_i64(self) -> Self::AsI64;
    fn to_i128(self) -> Self::AsI128;
    fn to_isize(self) -> Self::AsISize;

    fn to_f32(self) -> Self::AsF32;
    fn to_f64(self) -> Self::AsF64;
}

impl<T, U8, U16, U32, U64, U128, USize, I8, I16, I32, I64, I128, ISize, F32, F64, const N: usize>
    Numeric for [T; N]
where
    T: Numeric<
            AsU8 = U8,
            AsU16 = U16,
            AsU32 = U32,
            AsU64 = U64,
            AsU128 = U128,
            AsUSize = USize,
            AsI8 = I8,
            AsI16 = I16,
            AsI32 = I32,
            AsI64 = I64,
            AsI128 = I128,
            AsISize = ISize,
            AsF32 = F32,
            AsF64 = F64,
        >,
{
    type AsU8 = [U8; N];
    type AsU16 = [U16; N];
    type AsU32 = [U32; N];
    type AsU64 = [U64; N];
    type AsU128 = [U128; N];
    type AsUSize = [USize; N];
    type AsI8 = [I8; N];
    type AsI16 = [I16; N];
    type AsI32 = [I32; N];
    type AsI64 = [I64; N];
    type AsI128 = [I128; N];
    type AsISize = [ISize; N];
    type AsF32 = [F32; N];
    type AsF64 = [F64; N];

    #[inline]
    fn to_u8(self) -> Self::AsU8 {
        self.map(|x| x.to_u8())
    }

    #[inline]
    fn to_u16(self) -> Self::AsU16 {
        self.map(|x| x.to_u16())
    }

    #[inline]
    fn to_u32(self) -> Self::AsU32 {
        self.map(|x| x.to_u32())
    }

    #[inline]
    fn to_u64(self) -> Self::AsU64 {
        self.map(|x| x.to_u64())
    }

    #[inline]
    fn to_u128(self) -> Self::AsU128 {
        self.map(|x| x.to_u128())
    }

    #[inline]
    fn to_usize(self) -> Self::AsUSize {
        self.map(|x| x.to_usize())
    }

    #[inline]
    fn to_i8(self) -> Self::AsI8 {
        self.map(|x| x.to_i8())
    }

    #[inline]
    fn to_i16(self) -> Self::AsI16 {
        self.map(|x| x.to_i16())
    }

    #[inline]
    fn to_i32(self) -> Self::AsI32 {
        self.map(|x| x.to_i32())
    }

    #[inline]
    fn to_i64(self) -> Self::AsI64 {
        self.map(|x| x.to_i64())
    }

    #[inline]
    fn to_i128(self) -> Self::AsI128 {
        self.map(|x| x.to_i128())
    }

    #[inline]
    fn to_isize(self) -> Self::AsISize {
        self.map(|x| x.to_isize())
    }

    #[inline]
    fn to_f32(self) -> Self::AsF32 {
        self.map(|x| x.to_f32())
    }

    #[inline]
    fn to_f64(self) -> Self::AsF64 {
        self.map(|x| x.to_f64())
    }
}

/// A primitive number type.
pub trait Num:
    private::Sealed
    + Numeric<
        AsU8 = u8,
        AsU16 = u16,
        AsU32 = u32,
        AsU64 = u64,
        AsU128 = u128,
        AsUSize = usize,
        AsI8 = i8,
        AsI16 = i16,
        AsI32 = i32,
        AsI64 = i64,
        AsI128 = i128,
        AsISize = isize,
        AsF32 = f32,
        AsF64 = f64,
    > + Debug
    + Display
    + Copy
    + Clone
    + PartialEq<Self>
    + PartialOrd<Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Rem<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
    const FOUR: Self;
    const MIN: Self;
    const MAX: Self;

    /// Returns the minimum of the numbers.
    #[inline]
    fn min(a: Self, b: Self) -> Self {
        if a < b { a } else { b }
    }

    /// Returns the maximum of the numbers.
    #[inline]
    fn max(a: Self, b: Self) -> Self {
        if a > b { a } else { b }
    }

    /// Returns the two numbers as a tuple in ascending order.
    #[inline]
    fn min_max(a: Self, b: Self) -> (Self, Self) {
        if a < b { (a, b) } else { (b, a) }
    }

    /// Clamps `x` between `min` and `max`.
    #[inline]
    fn clamp(x: Self, min: Self, max: Self) -> Self {
        Self::max(Self::min(x, max), min)
    }
}

/// A signed primitive number type.
pub trait Signed: Num + Neg<Output = Self> {
    const NEG_ONE: Self;

    /// Computes the absolute value of the number.
    fn abs(x: Self) -> Self;

    /// Returns the sign of the number, or `0` if the number is `0`.
    fn signum(x: Self) -> Self;
}

/// A primitive floating point number type.
#[allow(clippy::wrong_self_convention)]
pub trait Float: Signed {
    const PI: Self;
    const PI_OVER_2: Self;
    const PI_OVER_4: Self;
    const TAU: Self;
    const SQRT_2: Self;
    const ONE_OVER_SQRT_2: Self;
    const NEG_ONE_OVER_SQRT_2: Self;
    const NEG_SQRT_2: Self;
    const EIGHTH: Self;
    const QUARTER: Self;
    const HALF: Self;
    const NUM_30: Self;
    const NUM_45: Self;
    const NUM_60: Self;
    const NUM_90: Self;
    const NUM_120: Self;
    const NUM_135: Self;
    const NUM_180: Self;
    const NUM_240: Self;
    const NUM_255: Self;
    const NUM_300: Self;
    const NUM_360: Self;

    /// Round to the nearest integer.
    fn round(x: Self) -> Self;

    /// Round down to the nearest integer.
    fn floor(x: Self) -> Self;

    /// Round up to the nearest integer.
    fn ceil(x: Self) -> Self;

    /// Return the number's square root.
    fn sqrt(x: Self) -> Self;

    /// Convert from degrees to radians.
    fn to_radians(x: Self) -> Self;

    /// Convert from radians to degrees.
    fn to_degrees(x: Self) -> Self;

    /// Computes the four quadrant arctangent of `y` and `x` in radians.
    fn atan2(y: Self, x: Self) -> Self;

    /// Simultaneously computes the sine and cosine of the number, x.
    /// Returns `(sin(x), cos(x))`.
    fn sin_cos(x: Self) -> (Self, Self);

    /// Returns the integer part of the number. This means that non-integer numbers are always
    /// truncated towards zero.
    fn trunc(x: Self) -> Self;

    /// Returns e^(x), (the exponential function).
    fn exp(x: Self) -> Self;
}

macro_rules! impl_num {
    (
        $($name:ident)*:
            ZERO = $zero:literal
            ONE = $one:literal
            TWO = $two:literal
            THREE = $three:literal
            FOUR = $four:literal
            ORD = $ord:ident
    ) => {
        $(
            impl Numeric for $name {
                type AsU8 = u8;
                type AsU16 = u16;
                type AsU32 = u32;
                type AsU64 = u64;
                type AsU128 = u128;
                type AsUSize = usize;
                type AsI8 = i8;
                type AsI16 = i16;
                type AsI32 = i32;
                type AsI64 = i64;
                type AsI128 = i128;
                type AsISize = isize;
                type AsF32 = f32;
                type AsF64 = f64;

                #[inline]
                fn to_u8(self) -> u8 { self as u8 }

                #[inline]
                fn to_u16(self) -> u16 { self as u16 }

                #[inline]
                fn to_u32(self) -> u32 { self as u32 }

                #[inline]
                fn to_u64(self) -> u64 { self as u64 }

                #[inline]
                fn to_u128(self) -> u128 { self as u128 }

                #[inline]
                fn to_usize(self) -> usize { self as usize }

                #[inline]
                fn to_i8(self) -> i8 { self as i8 }

                #[inline]
                fn to_i16(self) -> i16 { self as i16 }

                #[inline]
                fn to_i32(self) -> i32 { self as i32 }

                #[inline]
                fn to_i64(self) -> i64 { self as i64 }

                #[inline]
                fn to_i128(self) -> i128 { self as i128}

                #[inline]
                fn to_isize(self) -> isize { self as isize }

                #[inline]
                fn to_f32(self) -> f32 { self as f32 }

                #[inline]
                fn to_f64(self) -> f64 { self as f64 }
            }

            impl Num for $name {
                const ZERO: Self = $zero;
                const ONE: Self = $one;
                const TWO: Self = $two;
                const THREE: Self = $three;
                const FOUR: Self = $four;
                const MIN: Self = $name::MIN;
                const MAX: Self = $name::MAX;
            }
        )*
    };
}

macro_rules! impl_signed {
    (
        $($name:ident)*:
            ZERO = $zero:literal
            ONE = $one:literal
            TWO = $two:literal
            THREE = $three:literal
            FOUR = $four:literal
            ORD = $ord:ident
    ) => {
        impl_num!(
            $($name)*:
                ZERO = $zero
                ONE = $one
                TWO = $two
                THREE = $three
                FOUR = $four
                ORD = $ord
        );

        $(
            impl Signed for $name {
                const NEG_ONE: Self = -$one;

                #[inline]
                fn abs(x: Self) -> Self { x.abs() }

                #[inline]
                fn signum(x: Self) -> Self { x.signum() }
            }
        )*
    };
}

macro_rules! impl_float {
    ($($name:ident)*) => {
        impl_signed!(
            $($name)*:
                ZERO = 0.0
                ONE = 1.0
                TWO = 2.0
                THREE = 3.0
                FOUR = 4.0
                ORD = PartialOrd
        );

        $(
            impl Float for $name {
                const PI: Self = std::$name::consts::PI;
                const PI_OVER_2: Self = std::$name::consts::FRAC_PI_2;
                const PI_OVER_4: Self = std::$name::consts::FRAC_PI_4;
                const TAU: Self = std::$name::consts::TAU;
                const SQRT_2: Self = std::$name::consts::SQRT_2;
                const ONE_OVER_SQRT_2: Self = std::$name::consts::FRAC_1_SQRT_2;
                const NEG_ONE_OVER_SQRT_2: Self = -std::$name::consts::FRAC_1_SQRT_2;
                const NEG_SQRT_2: Self = -std::$name::consts::SQRT_2;
                const EIGHTH: Self = 0.125;
                const QUARTER: Self = 0.25;
                const HALF: Self = 0.5;
                const NUM_30: Self = 30.0;
                const NUM_45: Self = 45.0;
                const NUM_60: Self = 60.0;
                const NUM_90: Self = 90.0;
                const NUM_120: Self = 120.0;
                const NUM_135: Self = 135.0;
                const NUM_180: Self = 180.0;
                const NUM_240: Self = 240.0;
                const NUM_255: Self = 255.0;
                const NUM_300: Self = 300.0;
                const NUM_360: Self = 360.0;

                #[inline]
                fn round(x: Self) -> Self { x.round() }

                #[inline]
                fn floor(x: Self) -> Self { x.floor() }

                #[inline]
                fn ceil(x: Self) -> Self { x.ceil() }

                #[inline]
                fn sqrt(x: Self) -> Self { x.sqrt() }

                #[inline]
                fn to_radians(x: Self) -> Self { x.to_radians() }

                #[inline]
                fn to_degrees(x: Self) -> Self { x.to_degrees() }

                #[inline]
                fn atan2(y: Self, x: Self) -> Self { y.atan2(x) }

                #[inline]
                fn sin_cos(x: Self) -> (Self, Self) { x.sin_cos() }

                #[inline]
                fn trunc(x: Self) -> Self { x.trunc() }

                #[inline]
                fn exp(x: Self) -> Self { x.exp() }
            }
        )*
    };
}

/// A type that can approach a target.
pub trait Approach {
    /// The distance factor by which this type can approach its target.
    type Factor;

    /// Approach the target by the provided amount without overshooting.
    fn approach(self, target: Self, amount: Self::Factor) -> Self;
}

/// A type that can be interpolated.
pub trait Interp {
    /// The factor by which this type is interpolated.
    type Factor;

    /// Linear interpolation.
    fn lerp(self, target: Self, t: Self::Factor) -> Self;

    /// Quadratic bezier interpolation.
    fn quad_bezier(self, control: Self, target: Self, t: Self::Factor) -> Self;

    /// Cubic bezier interpolation.
    fn cubic_bezier(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self;

    /// Cubic Hermite[^wiki] interpolation.
    ///
    /// [^wiki]: <https://en.wikipedia.org/wiki/Cubic_Hermite_spline>
    fn hermite(
        self,
        tangent1: Self::Factor,
        target: Self,
        tangent2: Self::Factor,
        t: Self::Factor,
    ) -> Self;

    /// Catmull-Rom[^wiki] interpolation.
    ///
    /// [^wiki]: <https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Catmull%E2%80%93Rom_spline>
    fn catmull_rom(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self;

    /// Smooth-step[^wiki] interpolation.
    ///
    /// [^wiki]: <https://en.wikipedia.org/wiki/Smoothstep>
    fn smooth_step(self, target: Self, t: Self::Factor) -> Self;
}

/// A type that can smooth damp towards a target.
pub trait SmoothInterp: Interp {
    /// Accelerate towards a target with stateful velocity.
    fn smooth_damp(
        &mut self,
        velocity: &mut Self,
        target: Self,
        smooth_time: Self::Factor,
        max_speed: Self::Factor,
        delta_time: Self::Factor,
    );

    /// Lerp towards a target with a framerate-invariant version.
    ///
    /// See: [Lerp Smoothing is Broken](https://www.youtube.com/watch?v=LSNQuFEDOyQ).
    fn smooth_lerp(self, target: Self, t: Self::Factor, dt: Self::Factor) -> Self;
}

macro_rules! impl_approach {
    ($($type:ty)*) => {
        $(impl Approach for $type {
            type Factor = Self;

            #[inline]
            fn approach(self, target: Self, amount: Self::Factor) -> Self {
                if target >= self {
                    self.saturating_add(amount).min(target)
                } else {
                    self.saturating_sub(amount).max(target)
                }
            }
        })*
    };
}

macro_rules! impl_approach_f {
    ($($type:ty)*) => {
        $(impl Approach for $type {
            type Factor = Self;

            #[inline]
            fn approach(self, target: Self, amount: Self) -> Self {
                let diff = target - self;
                if diff.abs() <= amount {
                    target
                } else {
                    self + diff.signum() * amount
                }
            }
        })*
    };
}

macro_rules! impl_interp {
    ($($type:ty)*) => {
        $(
        impl Interp for $type {
            type Factor = Self;

            #[inline]
            fn lerp(self, target: Self, t: Self::Factor) -> Self {
                self * (1.0 - t) + target * t
            }

            #[inline]
            fn quad_bezier(self, control: Self, target: Self, t: Self::Factor) -> Self {
                self * (1.0 - t) * (1.0 - t) + control * 2.0 * (1.0 - t) * t + target * t * t
            }

            #[inline]
            fn cubic_bezier(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self {
                t * t * t * (target + 3.0 * (control1 - control2) - self)
                    + 3.0 * t * t * (self - 2.0 * control1 + control2)
                    + 3.0 * t * (control1 - self)
                    + self
            }

            #[inline]
            fn hermite(
                self,
                tangent1: Self::Factor,
                target: Self,
                tangent2: Self::Factor,
                t: Self::Factor,
            ) -> Self {
                (2.0 * self - 2.0 * target + tangent2 + tangent1) * t * t * t
                    + (3.0 * target - 3.0 * self - 2.0 * tangent1 - tangent2) * t * t
                    + tangent1 * t
                    + self
            }

            #[inline]
            fn catmull_rom(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self {
                0.5 * (2.0 * control1
                    + (control2 - self) * t
                    + (2.0 * self - 5.0 * control1 + 4.0 * control2 - target) * t * t
                    + (3.0 * control1 - self - 3.0 * control2 + target) * t * t * t)
            }

            #[inline]
            fn smooth_step(self, target: Self, t: Self::Factor) -> Self {
                self.hermite(0.0, target, 0.0, t)
            }
        }

        //////public static double DeltaRate(double v, double dt) {
        //         //     //     return Math.Exp(-v * dt);
        //         //     //   }
        //         //     //
        //         //     //   public static double LerpDelta(double a, double b, double v, double dt) {
        //         //     //     return Lerp(a, b, 1 - DeltaRate(dt, v));
        //         //     //   }
        //         //     //
        //         //     //   public static Vec LerpDelta(Vec a, Vec b, double v, double dt) {
        //         //     //     return Lerp(a, b, 1 - DeltaRate(dt, v));
        //         //     //   }

        impl SmoothInterp for $type {
            #[inline]
            fn smooth_damp(
                &mut self,
                velocity: &mut Self,
                target: Self,
                smooth_time: Self::Factor,
                max_speed: Self::Factor,
                delta_time: Self::Factor,
            ) {
                let smooth_time = smooth_time.max(0.0001);
                let omega = 2.0 / smooth_time;
                let x = omega * delta_time;
                let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
                let max_change = max_speed * smooth_time;
                let change = (*self - target).clamp(-max_change, max_change);
                let target = *self - change;
                let temp = (*velocity + omega * change) * delta_time;
                *velocity = (*velocity - omega * temp) * exp;
                let output = target + (change + temp) * exp;
                if (target - *self > 0.0) == (output > target) {
                    *velocity = (output - target) / delta_time;
                    *self = target;
                } else {
                    *self = output;
                }
            }

            #[inline]
            fn smooth_lerp(self, target: Self, t: Self::Factor, dt: Self::Factor) -> Self {
                let rate = (-t * dt).exp();
                self.lerp(target, 1.0 - rate)
            }
        }
        )*
    }
}

impl_num!(
    u8 u16 u32 u64 u128 usize:
        ZERO = 0
        ONE = 1
        TWO = 2
        THREE = 3
        FOUR = 4
        ORD = Ord
);

impl_signed!(
    i8 i16 i32 i64 i128 isize:
        ZERO = 0
        ONE = 1
        TWO = 2
        THREE = 3
        FOUR = 4
        ORD = Ord
);

impl_float!(f32 f64);

impl_approach!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl_approach_f!(f32 f64);

impl_interp!(f32 f64);
