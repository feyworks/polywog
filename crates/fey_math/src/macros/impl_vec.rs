macro_rules! impl_vec {
    (
        NAME = $name:ident
        SHORT = $short:ident
        LEN = $len:literal
        FIELDS = ($($p:ident),*)
        TUPLE = $tuple:tt
    ) => {
        impl<T> $name<T> {
            /// Create a new vector.
            #[inline]
            pub const fn new($($p: T,)*) -> Self {
                Self { $($p,)* }
            }

            /// Map the components of this vector.
            #[inline]
            pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> $name<U> {
                $short($(f(self.$p),)*)
            }

            /// Create a new vector with all components set to `value`.
            #[inline]
            pub fn splat(value: T) -> Self
            where
                T: Copy
            {
                Self { $($p: value,)* }
            }

            /// Returns a copy of this vector with absolute value of all components.
            #[inline]
            pub fn abs(self) -> Self
            where
                T: $crate::Signed
            {
                $short($(T::abs(self.$p),)*)
            }

            /// Returns a copy of this vector with signed value of all components.
            #[inline]
            pub fn signum(self) -> Self
            where
                T: $crate::Signed
            {
                $short($(T::signum(self.$p),)*)
            }

            /// Returns true if all components are set to zero.
            #[inline]
            pub fn is_zero(self) -> bool
            where
                T: $crate::Num
            {
                $(self.$p == T::ZERO &&)* true
            }

            /// Returns a new value where each component is set to the smallest value of
            /// each of the supplied vector's corresponding components.
            #[inline]
            pub fn min(self, rhs: Self) -> Self
            where
                T: $crate::Num
            {
                $short($(T::min(self.$p, rhs.$p),)*)
            }

            /// Returns a new value where each component is set to the greatest value of
            /// each of the supplied vector's corresponding components.
            #[inline]
            pub fn max(self, rhs: Self) -> Self
            where
                T: $crate::Num
            {
                $short($(T::max(self.$p, rhs.$p),)*)
            }

            /// Clamps the vector's components within the provided bounds.
            #[inline]
            pub fn clamp(self, min: Self, max: Self) -> Self
            where
                T: $crate::Num
            {
                $short($(T::clamp(self.$p, min.$p, max.$p),)*)
            }

            /// Rounds the components of this vector.
            #[inline]
            pub fn round(self) -> Self
            where
                T: $crate::Float
            {
                $short($(T::round(self.$p),)*)
            }

            /// Rounds down the components of this vector.
            #[inline]
            pub fn floor(self) -> Self
            where
                T: $crate::Float
            {
                $short($(T::floor(self.$p),)*)
            }

            /// Rounds up the components of this vector.
            #[inline]
            pub fn ceil(self) -> Self
            where
                T: $crate::Float
            {
                $short($(T::ceil(self.$p),)*)
            }

            /// Truncates the components of this vector. This means that non-integer
            /// numbers are always truncated towards zero.
            #[inline]
            pub fn trunc(self) -> Self
            where
                T: $crate::Float
            {
                $short($(T::trunc(self.$p),)*)
            }

            /// The dot product of this vector and another.
            #[inline]
            pub fn dot(self, other: Self) -> T
            where
                T: $crate::Num
            {
                T::ZERO $(+ self.$p * other.$p)*
            }

            /// The squared length of this vector.
            #[inline]
            pub fn sqr_len(self) -> T
            where
                T: $crate::Num
            {
                self.dot(self)
            }

            /// The square distance between this vector and another.
            #[inline]
            pub fn sqr_dist(self, other: Self) -> T
            where
                T: $crate::Num
            {
                (self - other).sqr_len()
            }

            /// Reflects this vector off the provided normal.
            #[inline]
            pub fn reflect(self, normal: Self) -> Self
            where
                T: $crate::Num
            {
                self - normal * T::TWO * self.dot(normal)
            }

            /// The length of this vector.
            #[inline]
            pub fn len(&self) -> T
            where
                T: $crate::Float
            {
                T::sqrt(self.sqr_len())
            }

            /// The distance between this vector and another.
            #[inline]
            pub fn dist(self, other: Self) -> T
            where
                T: $crate::Float
            {
                (self - other).len()
            }

            /// Returns a normalized version of this vector.
            #[inline]
            pub fn norm(self) -> Self
            where
                T: $crate::Float
            {
                let len = self.len();
                $short($(self.$p / len,)*)
            }

            /// Returns a normalized version of this vector, or a zeroed vector if
            /// the vector cannot be normalized.
            #[inline]
            pub fn norm_safe(self) -> Self
            where
                T: $crate::Float
            {
                let len = self.len();
                if len == T::ZERO {
                    return self;
                }
                $short($(self.$p / len,)*)
            }

            /// Returns a copy of this vector resized to be the provided length.
            #[inline]
            pub fn len_to(self, new_len: T) -> Self
            where
                T: $crate::Float
            {
                let f = new_len / self.len();
                $short($(self.$p * f,)*)
            }

            /// Returns a copy of this vector resized to be the provided length, or a
            /// zeroed vector if the original could not be normalized.
            #[inline]
            pub fn len_to_safe(self, new_len: T) -> Self
            where
                T: $crate::Float
            {
                let len = self.len();
                if len == T::ZERO {
                    return self;
                }
                let f = new_len / len;
                $short($(self.$p * f,)*)
            }

            /// Returns the normalized direction towards the other point.
            #[inline]
            pub fn dir_to(self, other: Self) -> Self
            where
                T: $crate::Float
            {
                (other - self).norm()
            }
        }

        impl<T: $crate::Num> $name<T> {
            pub const ZERO: Self = Self { $($p: T::ZERO,)* };
            pub const ONE: Self = Self { $($p: T::ONE,)* };
            pub const MIN: Self = Self { $($p: T::MIN,)* };
            pub const MAX: Self = Self { $($p: T::MAX,)* };
        }

        impl<T: $crate::Float> $crate::Approach for $name<T> {
            type Factor = T;

            #[inline]
            fn approach(self, target: Self, amount: Self::Factor) -> Self {
                let d = target - self;
                if d.sqr_len() >= amount * amount {
                    target
                } else {
                    self + d.norm_safe() * amount
                }
            }
        }

        $crate::impl_approx!(
            NAME = $name
            FIELDS = ($($p),*)
        );
        $crate::impl_tuple_arr!(
            NAME = $name
            LEN = $len
            FIELDS = ($($p),*)
            TUPLE = $tuple
        );
        $crate::impl_serde!(
            NAME = $name
            FIELDS = ($($p),*)
        );
        $crate::impl_bytemuck!($name);
        $crate::impl_casts!(
            NAME = $name
            FIELDS = ($($p),*)
        );
        $crate::impl_interp!(
            NAME = $name
            FIELDS = ($($p),*)
        );
        $crate::impl_op!($name Add add AddAssign add_assign $($p)*);
        $crate::impl_op!($name Sub sub SubAssign sub_assign $($p)*);
        $crate::impl_op!($name Mul mul MulAssign mul_assign $($p)*);
        $crate::impl_op!($name Div div DivAssign div_assign $($p)*);
        $crate::impl_op!($name Rem rem RemAssign rem_assign $($p)*);
        $crate::impl_op_scalar!($name Mul mul MulAssign mul_assign $($p)*);
        $crate::impl_op_scalar!($name Div div DivAssign div_assign $($p)*);
        $crate::impl_op_scalar!($name Rem rem RemAssign rem_assign $($p)*);
        $crate::impl_neg!($name $($p)*);
    };
}

pub(crate) use impl_vec;
