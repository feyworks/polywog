/// Implement `approx` traits on a type.
#[macro_export]
macro_rules! impl_approx {
    (
        NAME = $name:ident
        FIELDS = ($($p:tt),*)
    ) => {
        // ---------- APPROX ----------

        impl<T: Copy + approx::AbsDiffEq<Epsilon = T>> $name<T> {
            /// Returns true if the two values are approximately equal
            /// according to the absolute difference between their components.
            #[inline]
            pub fn abs_diff_eq(&self, other: &Self) -> bool {
                approx::abs_diff_eq!(self, other)
            }
        }

        impl<T> approx::AbsDiffEq for $name<T>
        where
            T: approx::AbsDiffEq,
            T::Epsilon: Copy,
        {
            type Epsilon = T::Epsilon;

            #[inline]
            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }

            #[inline]
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                true $(&& self.$p.abs_diff_eq(&other.$p, epsilon))*
            }
        }

        impl<T: Copy + approx::RelativeEq<Epsilon = T>> $name<T> {
            /// Returns true if the two values are approximately equal
            /// according to the absolute difference between their components,
            /// as well as relative-based comparisons.
            #[inline]
            pub fn relative_eq(&self, other: &Self) -> bool {
                approx::relative_eq!(self, other)
            }
        }

        impl<T> approx::RelativeEq for $name<T>
        where
            T: approx::RelativeEq,
            T::Epsilon: Copy,
        {
            #[inline]
            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative()
            }

            #[inline]
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                true $(&& self.$p.relative_eq(&other.$p, epsilon, max_relative))*
            }
        }

        impl<T: Copy + approx::UlpsEq<Epsilon = T>> $name<T> {
            /// Returns true if the two values are approximately equal
            /// according to the absolute difference between their components,
            /// as well as ULPs (Units in Last Place).
            #[inline]
            pub fn ulps_eq(&self, other: &Self) -> bool {
                approx::ulps_eq!(self, other)
            }
        }

        impl<T> approx::UlpsEq for $name<T>
        where
            T: approx::UlpsEq,
            T::Epsilon: Copy,
        {
            #[inline]
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                true $(&& self.$p.ulps_eq(&other.$p, epsilon, max_ulps))*
            }
        }
    }
}

pub use impl_approx;
