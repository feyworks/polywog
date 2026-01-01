#[macro_export]
macro_rules! impl_bytemuck {
    ($name:ident) => {
        unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for $name<T> {}
        unsafe impl<T: bytemuck::Pod> bytemuck::Pod for $name<T> {}

        impl<T: bytemuck::Pod + bytemuck::NoUninit> $name<T> {
            /// This type re-interpreted as a slice of bytes.
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                bytemuck::bytes_of(self)
            }

            /// This type re-interpreted as a mutable slice of bytes.
            #[inline]
            pub fn as_bytes_mut(&mut self) -> &mut [u8] {
                bytemuck::bytes_of_mut(self)
            }
        }
    };
}

pub use impl_bytemuck;
