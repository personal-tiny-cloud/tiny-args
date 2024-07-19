#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn arg_name_equality() {
        let flag = ArgName::Both {
            short: 'o',
            long: "out",
        };
        let flag1 = ArgName::Short('o');
        let flag2 = ArgName::Long("out".into());
        assert_eq!(flag, flag);
        assert_eq!(flag1, flag1);
        assert_eq!(flag2, flag2);
        assert_eq!(flag, flag1);
        assert_eq!(flag, flag2);
        assert_ne!(flag1, flag2);
    }

    #[test]
    #[should_panic]
    fn test_command_fail() {
        Command::create("test", "A really cool failing test")
            .arg(ArgName::Short('V'), ArgType::Flag, "Program's version")
            .arg(
                ArgName::Long("path"),
                ArgType::Path,
                "Insert a path to blow it away",
            )
            .arg(
                ArgName::Both {
                    short: 'h',
                    long: "help",
                },
                ArgType::Flag,
                "Show this help",
            )
            .arg(ArgName::Long("help"), ArgType::Path, "Oopsie")
            .build();
    }

    fn test_command() -> SubCommand {
        Command::create("test", "A really cool test")
            .arg(ArgName::Short('V'), ArgType::Flag, "Program's version")
            .arg(ArgName::Long("path"), ArgType::Path, "Insert a path")
            .arg(ArgName::Long("num"), ArgType::Num, "Insert a number")
            .arg(ArgName::Long("float"), ArgType::Float, "Insert a float")
            .arg(
                ArgName::Long("idk"),
                ArgType::String,
                "Just insert something",
            )
            .arg(
                ArgName::Long("idk2"),
                ArgType::String,
                "Just insert something again",
            )
            .arg(
                ArgName::Both {
                    short: 'h',
                    long: "help",
                },
                ArgType::Flag,
                "Show this help",
            )
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .license(env!("CARGO_PKG_LICENSE"))
            .into()
    }

    #[test]
    #[should_panic]
    fn test_subcmd_fail() {
        Command::create("testception", "A really failed test inception")
            .arg(
                ArgName::Long("idk"),
                ArgType::String,
                "Just insert something",
            )
            .subcommand(test_command())
            .subcommand(test_command())
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .license(env!("CARGO_PKG_LICENSE"))
            .build();
    }

    #[test]
    fn test_subcmd() {
        let cmd = Command::create("testception", "A really good test inception")
            .arg(
                ArgName::Long("idk"),
                ArgType::String,
                "Just insert something",
            )
            .subcommand(test_command())
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .license(env!("CARGO_PKG_LICENSE"))
            .build();
        println!("{}", cmd.help);
        let input: Vec<String> = vec![
            "test-program".into(),
            "test".into(),
            "--idk2".into(),
            "a b c".into(),
        ];
        let parsed = cmd.parse_from(input).unwrap();
        assert_eq!(
            parsed
                .args
                .get(ArgName::Long("idk2"))
                .unwrap()
                .value()
                .string(),
            "a b c"
        );
        assert!(parsed.help.starts_with("testception test"));
    }

    #[test]
    fn test_inputs() {
        let cmd = test_command().build();
        println!("{}", cmd.help);
        let input: Vec<String> = vec![
            "test-program".into(),
            "--num".into(),
            "6".into(),
            "--float".into(),
            "3.1415".into(),
            "-V".into(),
            "--path".into(),
            "/some/path".into(),
            "-h".into(),
            "--idk".into(),
            "hiii hello".into(),
        ];
        let parsed = cmd.parse_from(input).unwrap();
        assert_eq!(
            parsed.args.get(ArgName::Long("num")).unwrap().value().num(),
            6
        );
        assert_eq!(
            parsed
                .args
                .get(ArgName::Long("float"))
                .unwrap()
                .value()
                .float(),
            3.1415
        );
        assert_eq!(
            parsed
                .args
                .get(ArgName::Long("path"))
                .unwrap()
                .value()
                .path(),
            PathBuf::from("/some/path")
        );
        assert_eq!(
            parsed
                .args
                .get(ArgName::Long("idk"))
                .unwrap()
                .value()
                .string(),
            "hiii hello"
        );
        assert!(parsed
            .args
            .get(ArgName::Short::<char>('V'))
            .unwrap()
            .argvalue
            .is_some());
        assert!(parsed
            .args
            .get(ArgName::Short::<char>('h'))
            .unwrap()
            .argvalue
            .is_some());
        assert!(parsed.args.get(ArgName::Long("idk2")).is_none());
    }

    #[test]
    #[should_panic]
    fn test_bad_input1() {
        let cmd = test_command().build();
        let input: Vec<String> = vec![
            "test-program".into(),
            "--num".into(),
            "6".into(),
            "6".into(),
        ];
        cmd.parse_from(input).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_bad_input2() {
        let cmd = test_command().build();
        let input: Vec<String> = vec!["test-program".into(), "-num".into(), "6".into()];
        cmd.parse_from(input).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn test_bad_input3() {
        let cmd = test_command().build();
        let input: Vec<String> = vec!["test-program".into(), "num".into(), "6".into()];
        cmd.parse_from(input).unwrap();
    }

    #[test]
    fn test_tabbing() {
        let cmd = Command::create("tabbing", "Tests tabbing")
            .arg(
                ArgName::Long("aaaaaaa"),
                ArgType::Flag,
                "testestetstestestest",
            )
            .arg(
                ArgName::Long("aaaaaaaaaaa"),
                ArgType::Flag,
                "testestetstestestest",
            )
            .arg(
                ArgName::Long("aaaaaaaaaaaaaaa"),
                ArgType::Flag,
                "testestetstestestest",
            )
            .arg(
                ArgName::Long("aaaaaaaaaaaaaaaaaaa"),
                ArgType::Flag,
                "testestetstestestest",
            )
            .arg(
                ArgName::Long("aaaaaaaaaaaaaaaaaaaaaaaa"),
                ArgType::Flag,
                "testestetstestestest",
            )
            .subcommand(Command::create("testsub", "testsub").into())
            .subcommand(Command::create("testsubanan", "testsub").into())
            .subcommand(Command::create("testsubaaaaaaaa", "testsub").into())
            .subcommand(Command::create("testsubaaaaaaaaaaaaa", "testsub").into())
            .build();
        println!("{}", cmd.help);
    }
}
