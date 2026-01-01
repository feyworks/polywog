macro_rules! impl_angle {
    ($name:ident, $from:ident, $to:ident) => {
        $crate::impl_bytemuck!($name);
        $crate::impl_approx!(
            NAME = $name
            FIELDS = (0)
        );
        $crate::impl_casts!(
            NAME = $name
            FIELDS = (0)
        );

        $crate::impl_angle_op!($name $to Add add AddAssign add_assign);
        $crate::impl_angle_op!($name $to Sub sub SubAssign sub_assign);
        $crate::impl_angle_op!($name $to Mul mul MulAssign mul_assign);
        $crate::impl_angle_op!($name $to Div div DivAssign div_assign);
        $crate::impl_angle_op!($name $to Rem rem RemAssign rem_assign);
        $crate::impl_op_scalar!($name Add add AddAssign add_assign 0);
        $crate::impl_op_scalar!($name Sub sub SubAssign sub_assign 0);
        $crate::impl_op_scalar!($name Mul mul MulAssign mul_assign 0);
        $crate::impl_op_scalar!($name Div div DivAssign div_assign 0);
        $crate::impl_op_scalar!($name Rem rem RemAssign rem_assign 0);

        impl<T: $crate::Float> $crate::Direction<T> for $name<T> {
            #[inline]
            fn to_degrees(self) -> $crate::Degrees<T> {
                $crate::Degrees::$from(self)
            }

            #[inline]
            fn to_radians(self) -> $crate::Radians<T> {
                $crate::Radians::$from(self)
            }

            #[inline]
            fn to_rotations(self) -> $crate::Rotations<T> {
                $crate::Rotations::$from(self)
            }

            #[inline]
            fn from_vec2(v: $crate::Vec2<T>) -> Self {
                Self::from_radians($crate::Radians(T::atan2(v.y, v.x)))
            }

            fn sin_cos(self) -> (T, T) {
                T::sin_cos(self.to_radians().0)
            }

            #[inline]
            fn to_cardinal(self, _bias: $crate::Cardinal) -> $crate::Cardinal {
                $crate::Cardinal::from_vec2(self.norm())
            }

            #[inline]
            fn to_octal(self) -> $crate::Octal {
                $crate::Octal::from_vec2(self.norm())
            }
        }

        impl<T: $crate::Float> $crate::Angle<T> for $name<T> {}

        impl<T: $crate::Float> $name<T> {
            /// A zero-rotation angle.
            pub const ZERO: Self = Self(T::ZERO);

            /// If the angle's rotation is zero.
            #[inline]
            pub fn is_zero(self) -> bool {
                self.0 == T::ZERO
            }
        }

        impl<T: $crate::Float> From<$crate::Cardinal> for $name<T> {
            #[inline]
            fn from(value: $crate::Cardinal) -> Self {
                Self::from_cardinal(value)
            }
        }

        impl<T: $crate::Float> From<$crate::Octal> for $name<T> {
            #[inline]
            fn from(value: $crate::Octal) -> Self {
                Self::from_octal(value)
            }
        }
    };
}

pub(crate) use impl_angle;
