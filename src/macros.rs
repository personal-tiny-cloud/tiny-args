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
///
/// You can also specify long arguments with multiple words separated by dashes.
/// When doing this, it is recommended to use curly brackets (`arg! { }`) to keep
/// rustfmt from formatting the arguments.
///
/// # Example
///
/// ```rust
/// # use tiny_args::*;
/// assert_eq!(arg! { --long-help }, ArgName::Long("long-help"));
/// ```
#[macro_export]
macro_rules! arg {
    (-$short:ident, --$long:ident) => {{
        ArgName::Both {
            short: stringify!($short)
                .chars()
                .next()
                .unwrap(),
            long: stringify!($long),
        }
    }};

    (-$short:ident, --$first:ident$(-$long:ident)+) => {{
        ArgName::Both {
            short: stringify!($short)
                .chars()
                .next()
                .unwrap(),
            long: concat!(stringify!($first), $("-", stringify!($long),)+),
        }
    }};

    (-$short:ident) => {{
        ArgName::Short(
            stringify!($short)
                .chars()
                .next()
                .unwrap()
        )
    }};

    (--$long:ident) => {{
        ArgName::Long(stringify!($long))
    }};

    (--$first:ident$(-$long:ident)+) => {{
        ArgName::Long(concat!(stringify!($first), $("-", stringify!($long),)+))
    }};
}
