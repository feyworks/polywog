#[macro_export]
macro_rules! impl_op_scalar {
    (
        $name:ident
        $op_ty:ident
        $op_fn:ident
        $assign_ty:ident
        $assign_fn:ident
        $($p:tt)*
    ) => {
        impl<T: Copy + std::ops::$op_ty<T, Output = T>> std::ops::$op_ty<T> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn $op_fn(self, rhs: T) -> Self::Output {
                $name { $($p: self.$p.$op_fn(rhs),)* }
            }
        }

        impl<T: Copy + std::ops::$op_ty<T, Output = T>> std::ops::$op_ty<T> for &$name<T> {
            type Output = $name<T>;

            #[inline]
            fn $op_fn(self, rhs: T) -> Self::Output {
                $name { $($p: self.$p.$op_fn(rhs),)* }
            }
        }

        impl<T: Copy + std::ops::$assign_ty<T>> std::ops::$assign_ty<T> for $name<T> {
            #[inline]
            fn $assign_fn(&mut self, rhs: T) {
                $(self.$p.$assign_fn(rhs);)*
            }
        }

        impl<T: Copy + std::ops::$assign_ty<T>> std::ops::$assign_ty<&T> for $name<T> {
            #[inline]
            fn $assign_fn(&mut self, rhs: &T) {
                $(self.$p.$assign_fn(*rhs);)*
            }
        }
    }
}

pub use impl_op_scalar;
