#[macro_export]
macro_rules! __fluxer_gateway_bitflags_as_number {
    (
        $wrapper_name:ident =>
        $(#[$meta:meta])*
        $vis:vis struct $name:ident : $type:ty {
            $(const $item:ident = $num:expr ;)*
        }
    ) => {
        ::bitflags::bitflags! {
            $(#[$meta])*
            $vis struct $name : $type {
                $(const $item = $num ;)*
            }
        }

        #[derive(::serde::Serialize, ::serde::Deserialize, Debug, Clone, Copy)]
        #[serde(transparent)]
        $vis struct $wrapper_name($type);

        impl From<$name> for $wrapper_name {
            fn from(flags: $name) -> Self {
                Self(flags.bits())
            }
        }

        impl From<$wrapper_name> for $name {
            fn from(def: $wrapper_name) -> Self {
                <$name>::from_bits_retain(def.0)
            }
        }
    };
}
