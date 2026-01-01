#[macro_export]
macro_rules! impl_interp {
    (
        NAME = $name:ident
        FIELDS = ($($p:ident),*)
    ) => {
        impl<T: $crate::Float + $crate::Interp<Factor = T>> $crate::Interp for $name<T> {
            type Factor = T;

            #[inline]
            fn lerp(self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.lerp(target.$p, t),)* }
            }

            #[inline]
            fn quad_bezier(self, control: Self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.quad_bezier(control.$p, target.$p, t),)* }
            }

            #[inline]
            fn cubic_bezier(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.cubic_bezier(control1.$p, control2.$p, target.$p, t),)* }
            }

            #[inline]
            fn hermite(
                self,
                tangent1: Self::Factor,
                target: Self,
                tangent2: Self::Factor,
                t: Self::Factor,
            ) -> Self {
                $name { $($p: self.$p.hermite(tangent1, target.$p, tangent2, t),)* }
            }

            #[inline]
            fn catmull_rom(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.catmull_rom(control1.$p, control2.$p, target.$p, t),)* }
            }

            #[inline]
            fn smooth_step(self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.smooth_step(target.$p, t),)* }
            }
        }

        impl<T: $crate::Float + $crate::SmoothInterp<Factor = T>> $crate::SmoothInterp for $name<T> {
            #[inline]
            fn smooth_damp(
                &mut self,
                velocity: &mut Self,
                target: Self,
                smooth_time: Self::Factor,
                max_speed: Self::Factor,
                delta_time: Self::Factor,
            ) {
                $(self.$p.smooth_damp(&mut velocity.$p, target.$p, smooth_time, max_speed, delta_time);)*
            }

            #[inline]
            fn smooth_lerp(self, target: Self, t: Self::Factor, dt: Self::Factor) -> Self {
                let rate = T::exp(-t * dt);
                $crate::Interp::lerp(self, target, Self::Factor::ONE - rate)
            }
        }
    };
    (
        NAME = $name:ident
        FIELD_TY = $f:ident
        FIELDS = ($($p:ident),*)
    ) => {
        impl<T: $crate::Float> $crate::Interp for $name<T>
        where
            $f<T>: $crate::Interp<Factor = T>,
        {
            type Factor = T;

            #[inline]
            fn lerp(self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.lerp(target.$p, t),)* }
            }

            #[inline]
            fn quad_bezier(self, control: Self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.quad_bezier(control.$p, target.$p, t),)* }
            }

            #[inline]
            fn cubic_bezier(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.cubic_bezier(control1.$p, control2.$p, target.$p, t),)* }
            }

            #[inline]
            fn hermite(
                self,
                tangent1: Self::Factor,
                target: Self,
                tangent2: Self::Factor,
                t: Self::Factor,
            ) -> Self {
                $name { $($p: self.$p.hermite(tangent1, target.$p, tangent2, t),)* }
            }

            #[inline]
            fn catmull_rom(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.catmull_rom(control1.$p, control2.$p, target.$p, t),)* }
            }

            #[inline]
            fn smooth_step(self, target: Self, t: Self::Factor) -> Self {
                $name { $($p: self.$p.smooth_step(target.$p, t),)* }
            }
        }
    };
    (
        NAME = $name:ident
        INDICES = [$($i:literal),*]
    ) => {
        impl<T: $crate::Float> $crate::Interp for $name<T>
        where
            $crate::Vec2<T>: $crate::Interp<Factor = T>,
        {
            type Factor = T;

            #[inline]
            fn lerp(self, target: Self, t: Self::Factor) -> Self {
                $name([$(self.0[$i].lerp(target.0[$i], t)),*])
            }

            #[inline]
            fn quad_bezier(self, control: Self, target: Self, t: Self::Factor) -> Self {
                $name([$(self.0[$i].quad_bezier(control.0[$i], target.0[$i], t)),*])
            }

            #[inline]
            fn cubic_bezier(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self {
                $name([$(self.0[$i].cubic_bezier(control1.0[$i], control2.0[$i], target.0[$i], t)),*])
            }

            #[inline]
            fn hermite(
                self,
                tangent1: Self::Factor,
                target: Self,
                tangent2: Self::Factor,
                t: Self::Factor,
            ) -> Self {
                $name([$(self.0[$i].hermite(tangent1, target.0[$i], tangent2, t)),*])
            }

            #[inline]
            fn catmull_rom(self, control1: Self, control2: Self, target: Self, t: Self::Factor) -> Self {
                $name([$(self.0[$i].catmull_rom(control1.0[$i], control2.0[$i], target.0[$i], t)),*])
            }

            #[inline]
            fn smooth_step(self, target: Self, t: Self::Factor) -> Self {
                $name([$(self.0[$i].smooth_step(target.0[$i], t)),*])
            }
        }
    };
}

pub use impl_interp;
