#[macro_export]
macro_rules! serde_for_enum {
    (
        $name:ident {
            $(
                $variant:ident => $str:expr
            ),* $(,)?
        }
    ) => {
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        $name::$variant => serializer.collect_str($str),
                    )*
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                match String::deserialize(deserializer)?.as_str() {
                    $(
                        $str => Ok($name::$variant),
                    )*
                    other_value => Err(serde::de::Error::custom(format!(
                        "Unsupported {} [{}].",
                        stringify!($name),
                        &other_value
                    ))),
                }
            }
        }
    };
}
