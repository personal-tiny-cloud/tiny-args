#[cfg(doc)]
use crate::ArgName;

/// A shorthand macro for [`ArgName`].
///
/// Using this macro to create [`ArgName`]s is equivalent to using the enum builder.
///
/// # Example
/// 
/// ```rust
/// # use tiny_args::*;
/// assert_eq!(ArgName::Both { short: 'h', long: "help" }, arg!(-h, --help));
/// assert_eq!(ArgName::Short::<&str>('h'), arg!(-h));
/// assert_eq!(ArgName::Long("help"), arg!(--help));
/// ```
#[macro_export]
macro_rules! arg {
    (-$short:ident, --$long:ident) => {{
        ArgName::Both {
            short: stringify!($short)
                .chars()
                .next()
                .expect("Tried to use an empty string as a short argument"),
            long: stringify!($long),
        }
    }};

    (-$short:ident) => {{
        ArgName::Short(
            stringify!($short)
                .chars()
                .next()
                .expect("Tried to use an empty string as a short argument"),
        )
    }};

    (--$long:ident) => {{
        ArgName::Long(stringify!($long))
    }};
}
