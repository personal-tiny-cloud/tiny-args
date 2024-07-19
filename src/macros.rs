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
