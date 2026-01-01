/// Impl from/to tuples and arrays for a simple struct.
#[macro_export]
macro_rules! impl_tuple_arr {
    (
        NAME = $name:ident
        LEN = $len:literal
        FIELDS = ($($p:ident),*)
        TUPLE = $tuple:tt
    ) => {
        impl<T> From<$tuple> for $name<T> {
            #[inline]
            fn from(($($p,)*): $tuple) -> Self {
                Self { $($p,)* }
            }
        }

        impl<T> From<$name<T>> for $tuple {
            #[inline]
            fn from($name { $($p,)* }: $name<T>) -> $tuple {
                ($($p,)*)
            }
        }

        impl<T> From<[T; $len]> for $name<T> {
            #[inline]
            fn from([$($p,)*]: [T; $len]) -> Self {
                Self { $($p,)* }
            }
        }

        impl<T> From<$name<T>> for [T; $len] {
            #[inline]
            fn from($name { $($p,)* }: $name<T>) -> [T; $len] {
                [$($p,)*]
            }
        }
    };
}

pub use impl_tuple_arr;
