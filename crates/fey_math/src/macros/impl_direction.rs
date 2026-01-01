macro_rules! impl_direction {
    ($name:ident, $len:literal, $($var:ident),*) => {
        impl $name {
            /// All direction variants.
            pub const VARIANTS: [Self; $len] = [$(Self::$var,)*];

            /// All direction names.
            pub const NAMES: [&'static str; $len] = [$(stringify!($var),)*];

            /// Parse from a direction's variant index.
            #[inline]
            pub fn from_index(idx: usize) -> Option<Self> {
                Self::VARIANTS.get(idx).copied()
            }

            /// String representation of this direction.
            #[inline]
            pub fn to_str(self) -> &'static str {
                match self {
                    $(Self::$var => stringify!($var),)*
                }
            }

            /// Parse from a string representation, for example `"East"`.
            #[inline]
            pub fn from_str(s: &str) -> Option<Self> {
                Some(match s {
                    $(stringify!($var) => Self::$var,)*
                    _ => return None,
                })
            }
        }

        impl std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self.to_str(), f)
            }
        }
    };
}

pub(crate) use impl_direction;
