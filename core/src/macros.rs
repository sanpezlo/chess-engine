/// A macro for creating an enum with some additional utility functions.
macro_rules! create_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident
            ),*
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant,
            )*
        }

        impl $name {
            #[doc = concat!("The number of variants in the `", stringify!($name), "` enum.")]
            pub const LEN: usize = [$($name::$variant),*].len();

            #[doc = concat!("An array of all the variants in the `", stringify!($name), "` enum.")]
            pub const ALL: [Self; Self::LEN] = [$($name::$variant),*];

            #[doc = concat!(
                "Creates a new `", stringify!($name), "` from an index.\n",
                "# Panics\n",
                "Panics if the index is out of bounds."
            )]
            #[inline(always)]
            pub const fn new(index: usize) -> Self {

                $(
                    #[allow(non_upper_case_globals)]
                    pub const $variant: usize = $name::$variant as usize;
                )*

                #[allow(non_upper_case_globals)]
                match index {
                    $($variant => $name::$variant,)*
                    _ => unreachable!(),
                }
            }
        }
    };
}

macro_rules! enum_str {
    ($name:ident, $error:ident {
        $($variant:ident = $str:expr),*
    }) => {
        #[doc = concat!(
            "An error that can occur when parsing a [`",
            stringify!($name),
            "`]."
        )]
        #[derive(thiserror::Error, Debug)]
        #[error(
            "invalid {} (expected {}, got {0})",
            stringify!($name),
            $crate::macros::expected!($($str),*)
        )]
        pub struct $error(pub String);

        #[doc = concat!(
            "Parses a `", stringify!($name), "` from a string.\n",
            "# Errors\n",
            "Returns a [`", stringify!($error), "`] if the string is not a valid ", stringify!($name), "."
        )]
        impl std::str::FromStr for $name {
            type Err = $error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($str => Ok($name::$variant),)*
                    _ => Err($error(s.to_string())),
                }
            }
        }

        #[doc = concat!("Formats a `", stringify!($name), "` as a string.")]
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let s = match self {
                    $($name::$variant => $str,)*
                };
                write!(f, "{}", s)
            }
        }
    };
}

macro_rules! expected {
    ($str:expr, $($strs:expr),*) => {
        concat!($str, "-", $crate::macros::tail!($($strs),*))
    };
    ($str:expr) => {
        $str
    };
}

macro_rules! tail {
    ($head:expr, $($tail:expr),*) => {
        $crate::macros::tail!($($tail),*)
    };
    ($tail:expr) => {
        $tail
    };
}

pub(crate) use create_enum;
pub(crate) use enum_str;
pub(crate) use expected;
pub(crate) use tail;
