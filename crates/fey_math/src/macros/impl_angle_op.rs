macro_rules! impl_angle_op {
    (
        $name:ident
        $to_fn:ident
        $op_ty:ident
        $op_fn:ident
        $assign_ty:ident
        $assign_fn:ident
    ) => {
        $crate::impl_angle_op!($name $to_fn $op_ty $op_fn $assign_ty $assign_fn Degrees Radians Rotations);
    };
    (
        $name:ident
        $to_fn:ident
        $op_ty:ident
        $op_fn:ident
        $assign_ty:ident
        $assign_fn:ident
        $($rhs:ident)*
    ) => {
        $(
        impl<T: $crate::Float> std::ops::$op_ty<$crate::$rhs<T>> for $name<T> {
            type Output = $name<T>;
            #[inline]
            fn $op_fn(self, rhs: $crate::$rhs<T>) -> Self::Output {
                $name(self.0.$op_fn($crate::Direction::$to_fn(rhs).0))
            }
        }

        impl<T: $crate::Float> std::ops::$op_ty<$crate::$rhs<T>> for &$name<T> {
            type Output = $name<T>;
            #[inline]
            fn $op_fn(self, rhs: $crate::$rhs<T>) -> Self::Output {
                $name(self.0.$op_fn($crate::Direction::$to_fn(rhs).0))
            }
        }

        impl<T: $crate::Float> std::ops::$op_ty<&$crate::$rhs<T>> for $name<T> {
            type Output = $name<T>;
            #[inline]
            fn $op_fn(self, rhs: &$crate::$rhs<T>) -> Self::Output {
                $name(self.0.$op_fn($crate::Direction::$to_fn(*rhs).0))
            }
        }

        impl<T: $crate::Float> std::ops::$op_ty<&$crate::$rhs<T>> for &$name<T> {
            type Output = $name<T>;
            #[inline]
            fn $op_fn(self, rhs: &$crate::$rhs<T>) -> Self::Output {
                $name(self.0.$op_fn($crate::Direction::$to_fn(*rhs).0))
            }
        }

        impl<T: $crate::Float> std::ops::$assign_ty<$crate::$rhs<T>>
            for $name<T>
        {
            #[inline]
            fn $assign_fn(&mut self, rhs: $crate::$rhs<T>) {
                self.0.$assign_fn($crate::Direction::$to_fn(rhs).0)
            }
        }

        impl<T: $crate::Float> std::ops::$assign_ty<&$crate::$rhs<T>>
            for $name<T>
        {
            #[inline]
            fn $assign_fn(&mut self, rhs: &$crate::$rhs<T>) {
                self.0.$assign_fn($crate::Direction::$to_fn(*rhs).0)
            }
        }
        )*
    };
}

pub(crate) use impl_angle_op;
