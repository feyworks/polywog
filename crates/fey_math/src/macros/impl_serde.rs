#[macro_export]
macro_rules! impl_serde {
    (
        NAME = $name:ident
        FIELDS = ($($p:ident),*)
    ) => {
        impl<T: serde::Serialize> serde::Serialize for $name<T> {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                ($(&self.$p),*).serialize(serializer)
            }
        }

        impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for $name<T> {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let ($($p),*) = <_>::deserialize(deserializer)?;
                Ok(Self {$($p),*})
            }
        }
    };
    (
        NAME = $name:ident
        FIELDS = [$($p:ident),*]
    ) => {
        impl<T: serde::Serialize> serde::Serialize for $name<T> {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                [$(&self.$p),*].serialize(serializer)
            }
        }

        impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for $name<T> {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let [$($p),*] = <_>::deserialize(deserializer)?;
                Ok(Self {$($p),*})
            }
        }
    };
}

pub use impl_serde;
