macro_rules! impl_mat {
    (
        NAME = $name:ident
        SHORT = $short:ident
        VEC_TY = $vec:ident
        MUL_FN = $mul_fn:ident
        FIELDS = ($($p:ident),*)
        CONSTS = ($($c:ident),*)
    ) => {
        $crate::impl_serde!(
            NAME = $name
            FIELDS = ($($p),*)
        );
        $crate::impl_approx!(
            NAME = $name
            FIELDS = ($($p),*)
        );
        $crate::impl_bytemuck!($name);
        $crate::impl_casts!(
            NAME = $name
            FIELDS = ($($p),*)
        );

        $crate::impl_op_scalar!($name Mul mul MulAssign mul_assign $($p)*);
        $crate::impl_op_scalar!($name Div div DivAssign div_assign $($p)*);
        $crate::impl_op_scalar!($name Rem rem RemAssign rem_assign $($p)*);
        $crate::impl_neg!($name $($p)*);

        impl<T> $name<T> {
            /// Create a new matrix.
            #[inline]
            pub const fn new($($p: $vec<T>),*) -> Self {
                $short($($p),*)
            }
        }

        impl<T: $crate::Num> $name<T> {
            /// Matrix with all components set to zero.
            pub const ZERO: Self = Self { $($p: $vec::ZERO),* };

            /// Identity matrix.
            pub const IDENTITY: Self = $short($($vec::$c),*);
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

pub(crate) use impl_mat;
