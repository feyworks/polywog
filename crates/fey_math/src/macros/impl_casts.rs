/// Impl `Castable` on a generic type.
#[macro_export]
macro_rules! impl_casts {
    (
        NAME = $name:ident
        FIELDS = ($($p:tt),*)
    ) => {
        impl<T: $crate::Numeric<
            AsU8 = u8,
            AsU16 = u16,
            AsU32 = u32,
            AsU64 = u64,
            AsU128 = u128,
            AsUSize = usize,
            AsI8 = i8,
            AsI16 = i16,
            AsI32 = i32,
            AsI64 = i64,
            AsI128 = i128,
            AsISize = isize,
            AsF32 = f32,
            AsF64 = f64,
        >> $crate::Numeric for $name<T> {
            type AsU8 = $name<u8>;
            type AsU16 = $name<u16>;
            type AsU32 = $name<u32>;
            type AsU64 = $name<u64>;
            type AsU128 = $name<u128>;
            type AsUSize = $name<usize>;
            type AsI8 = $name<i8>;
            type AsI16 = $name<i16>;
            type AsI32 = $name<i32>;
            type AsI64 = $name<i64>;
            type AsI128 = $name<i128>;
            type AsISize = $name<isize>;
            type AsF32 = $name<f32>;
            type AsF64 = $name<f64>;

            #[inline]
            fn to_u8(self) -> $name<u8> { $name { $($p: self.$p.to_u8()),* } }

            #[inline]
            fn to_u16(self) -> $name<u16> {$name { $($p: self.$p.to_u16()),* } }

            #[inline]
            fn to_u32(self) -> $name<u32> { $name { $($p: self.$p.to_u32()),* } }

            #[inline]
            fn to_u64(self) -> $name<u64> { $name { $($p: self.$p.to_u64()),* } }

            #[inline]
            fn to_u128(self) -> $name<u128> { $name { $($p: self.$p.to_u128()),* } }

            #[inline]
            fn to_usize(self) -> $name<usize> { $name { $($p: self.$p.to_usize()),* } }

            #[inline]
            fn to_i8(self) -> $name<i8> { $name { $($p: self.$p.to_i8()),* } }

            #[inline]
            fn to_i16(self) -> $name<i16> {$name { $($p: self.$p.to_i16()),* } }

            #[inline]
            fn to_i32(self) -> $name<i32> { $name { $($p: self.$p.to_i32()),* } }

            #[inline]
            fn to_i64(self) -> $name<i64> { $name { $($p: self.$p.to_i64()),* } }

            #[inline]
            fn to_i128(self) -> $name<i128> { $name { $($p: self.$p.to_i128()),* } }

            #[inline]
            fn to_isize(self) -> $name<isize> { $name { $($p: self.$p.to_isize()),* } }

            #[inline]
            fn to_f32(self) -> $name<f32> { $name { $($p: self.$p.to_f32()),* } }

            #[inline]
            fn to_f64(self) -> $name<f64> { $name { $($p: self.$p.to_f64()),* } }
        }
    };
}

pub use impl_casts;
