#[macro_export]
macro_rules! impl_neg {
    ($name:ident $($p:ident)*) => {
        impl<T: std::ops::Neg<Output = T>> std::ops::Neg for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn neg(self) -> Self::Output {
                $name { $($p: -self.$p,)* }
            }
        }
    }
}

pub use impl_neg;
