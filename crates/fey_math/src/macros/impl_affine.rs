#[macro_export]
macro_rules! impl_affine {
    (
        NAME = $name:ident
        SHORT = $short:ident
        MUL_FN = $mul_fn:ident
        MAT_TY = $mat:ident
        VEC_TY = $vec:ident
    ) => {
        $crate::impl_bytemuck!($name);
        $crate::impl_approx!(
            NAME = $name
            FIELDS = (matrix, translation)
        );
        $crate::impl_serde!(
            NAME = $name
            FIELDS = (matrix, translation)
        );
        $crate::impl_casts!(
            NAME = $name
            FIELDS = (matrix, translation)
        );

        $crate::impl_op_scalar!($name Mul mul MulAssign mul_assign matrix translation);
        $crate::impl_op_scalar!($name Div div DivAssign div_assign matrix translation);
        $crate::impl_op_scalar!($name Rem rem RemAssign rem_assign matrix translation);
        $crate::impl_neg!($name matrix translation);

        impl<T> $name<T> {
            /// Create a new matrix.
            #[inline]
            pub const fn new(matrix: $mat<T>, translation: $vec<T>) -> Self {
                $short(matrix, translation)
            }
        }

        impl<T: $crate::Num> $name<T> {
            /// A zeroed matrix.
            pub const ZERO: Self = $short($mat::ZERO, $vec::ZERO);

            /// An identity matrix.
            pub const IDENTITY: Self = $short($mat::IDENTITY, $vec::ZERO);

             /// Create a translation matrix.
            #[inline]
            pub fn translation(amount: impl Into<$vec<T>>) -> Self {
                $short($mat::IDENTITY, amount.into())
            }

            /// Create a scaling matrix.
            #[inline]
            pub fn scale(scale: impl Into<$vec<T>>) -> Self {
                $short($mat::scale(scale), $vec::ZERO)
            }

            /// Create a uniform scaling matrix.
            #[inline]
            pub fn scale_of(scale: T) -> Self {
                $short($mat::scale($vec::splat(scale)), $vec::ZERO)
            }
        }

        impl<T: $crate::Num> Default for $name<T> {
            #[inline]
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl<T: $crate::Num> From<$mat<T>> for $name<T> {
            #[inline]
            fn from(value: $mat<T>) -> Self {
                $short(value, $vec::ZERO)
            }
        }

        impl<T: $crate::Float> std::ops::Mul<$name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn mul(self, rhs: $name<T>) -> Self::Output {
                self.$mul_fn(&rhs)
            }
        }

        impl<T: $crate::Float> std::ops::Mul<$name<T>> for &$name<T> {
            type Output = $name<T>;

            #[inline]
            fn mul(self, rhs: $name<T>) -> Self::Output {
                self.$mul_fn(&rhs)
            }
        }

        impl<T: $crate::Float> std::ops::Mul<&$name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn mul(self, rhs: &$name<T>) -> Self::Output {
                self.$mul_fn(&rhs)
            }
        }

         impl<T: $crate::Float> std::ops::Mul<&$name<T>> for &$name<T> {
            type Output = $name<T>;

            #[inline]
            fn mul(self, rhs: &$name<T>) -> Self::Output {
                self.$mul_fn(&rhs)
            }
        }
    };
}

pub use impl_affine;
