// This file is part of the Tiny Cloud project.
// You can find the source code of every repository here:
//		https://github.com/personal-tiny-cloud
//
// Copyright (C) 2024  hex0x0000
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Email: hex0x0000@protonmail.com

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
    fn test_arg_macro() {
        assert_eq!(
            arg! { -h, --help },
            ArgName::Both {
                short: 'h',
                long: "help"
            }
        );
        assert_eq!(arg! { -h, --long-help }, ArgName::Long("long-help"));
        assert_eq!(
            arg! { -h, --long-long-long-help },
            ArgName::Long("long-long-long-help")
        );
        assert_eq!(arg!(-h), ArgName::Short::<&str>('h'));
        assert_eq!(arg!(--help), ArgName::Long("help"));
        assert_eq!(arg! { --long-help }, ArgName::Long("long-help"));
        assert_eq!(
            arg! { --long-long-long-help },
            ArgName::Long("long-long-long-help")
        );
    }

    #[test]
    #[should_panic]
    fn test_command_fail() {
        Command::create("test", "A really cool failing test")
            .arg(arg!(-V), ArgType::Flag, "Program's version")
            .arg(arg!(--path), ArgType::Path, "Insert a path to blow it away")
            .arg(arg!(-h, --help), ArgType::Flag, "Show this help")
            .arg(arg!(--help), ArgType::Path, "Oopsie")
            .build();
    }

    fn test_command() -> SubCommand {
        Command::create("test", "A really cool test")
            .arg(arg!(-V), ArgType::Flag, "Program's version")
            .arg(arg!(--path), ArgType::Path, "Insert a path")
            .arg(arg!(--num), ArgType::Num, "Insert a number")
            .arg(arg!(--float), ArgType::Float, "Insert a float")
            .arg(arg!(--idk), ArgType::String, "Just insert something")
            .arg(arg!(--idk2), ArgType::String, "Just insert something again")
            .arg(arg!(-h, --help), ArgType::Flag, "Show this help")
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .license(env!("CARGO_PKG_LICENSE"))
            .into()
    }

    #[test]
    #[should_panic]
    fn test_subcmd_fail() {
        Command::create("testception", "A really failed test inception")
            .arg(arg!(--idk), ArgType::String, "Just insert something")
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
            .arg(arg!(--idk), ArgType::String, "Just insert something")
            .subcommand(test_command().color(false))
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
        assert_eq!(
            parsed.args.get(arg!(--idk2)).unwrap().description(),
            "Just insert something again"
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
        assert_eq!(parsed.args.get(arg!(--num)).unwrap().value().num(), 6);
        assert_eq!(
            parsed.args.get(arg!(--float)).unwrap().value().float(),
            3.1415
        );
        assert_eq!(
            parsed
                .args
                .get(arg!(--path))
                .unwrap()
                .value()
                .path()
                .clone(),
            PathBuf::from("/some/path")
        );
        assert_eq!(
            parsed.args.get(arg!(--idk)).unwrap().value().string(),
            "hiii hello"
        );
        assert!(parsed.args.contains(arg!(-V)));
        assert!(parsed.args.contains(arg!(-h)));
        assert!(!parsed.args.contains(arg!(--idk2)));
    }

    #[test]
    fn test_single_arg1() {
        let cmd = test_command().build();
        let input: Vec<String> = vec!["test-program".into(), "--help".into()];
        let parsed = cmd.parse_from(input).unwrap();
        assert!(parsed.args.contains(arg!(--help)));
        assert!(parsed.args.contains(arg!(-h)));
    }

    #[test]
    fn test_single_arg2() {
        let cmd = test_command().build();
        let input: Vec<String> = vec!["test-program".into(), "--float".into(), "3.1415".into()];
        let parsed = cmd.parse_from(input).unwrap();
        assert_eq!(
            parsed.args.get(arg!(--float)).unwrap().value().float(),
            3.1415
        );
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
    #[should_panic]
    fn test_bad_input4() {
        let cmd = test_command().build();
        let input: Vec<String> = vec![
            "test-program".into(),
            "--num".into(),
            "6".into(),
            "--num".into(),
            "6".into(),
        ];
        cmd.parse_from(input).unwrap();
    }

    #[test]
    fn test_tabbing() {
        let cmd = Command::create("tabbing", "Tests tabbing")
            .arg(arg!(--aaaaaaa), ArgType::Flag, "testestetstestestest")
            .arg(arg!(--aaaaaaaaaaa), ArgType::Flag, "testestetstestestest")
            .arg(
                arg!(--aaaaaaaaaaaaaaa),
                ArgType::Flag,
                "testestetstestestest",
            )
            .arg(
                arg!(--aaaaaaaaaaaaaaaaaaa),
                ArgType::Flag,
                "testestetstestestest",
            )
            .arg(
                arg!(--aaaaaaaaaaaaaaaaaaaaaaaa),
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
