use crate::*;

fn args(argslist: ArgList<String>, mut inputargs: Vec<String>) -> Result<ArgList<String>, String> {
    let mut argslist = argslist;
    let mut argopt: Option<Arg<String>> = None;
    while let Some(input) = inputargs.first() {
        if let Some(arg) = &argopt {
            match arg.argtype {
                ArgType::String => {
                    argslist.init_arg(&arg.argname, ArgValue::String(input.clone()));
                    inputargs.remove(0);
                }
                ArgType::Num => {
                    argslist.init_arg(
                        &arg.argname,
                        ArgValue::Num(input.parse().map_err(|e| {
                            format!("'{}' value's must be a valid number: {e}", arg.argname)
                        })?),
                    );
                    inputargs.remove(0);
                }
                ArgType::Float => {
                    argslist.init_arg(
                        &arg.argname,
                        ArgValue::Float(input.parse().map_err(|e| {
                            format!(
                                "'{}' value's must be a valid float number: {e}",
                                arg.argname
                            )
                        })?),
                    );
                    inputargs.remove(0);
                }
                ArgType::Path => {
                    argslist.init_arg(&arg.argname, ArgValue::Path(PathBuf::from(input)));
                    inputargs.remove(0);
                }
                ArgType::Flag => argslist.init_arg(&arg.argname, ArgValue::Flag),
            }
            argopt = None;
        } else if input.starts_with("--") {
            if let Some(input) = input.get(2..) {
                if let Some(foundarg) = argslist.get(ArgName::Long(input)) {
                    argopt = Some(foundarg);
                    inputargs.remove(0);
                } else {
                    return Err(format!("'--{input}' is not a valid argument."));
                }
            } else {
                return Err(format!("'{input}' is not a valid long argument."));
            }
        } else if input.starts_with('-') {
            if let Some(input) = input.chars().nth(1) {
                if let Some(foundarg) = argslist.get(ArgName::Short(input)) {
                    argopt = Some(foundarg);
                    inputargs.remove(0);
                } else {
                    return Err(format!("'-{input}' is not a valid argument."));
                }
            } else {
                return Err(format!("'{input}' is not a valid short argument."));
            }
        } else {
            return Err(format!("'{input}' is not an argument nor a value."));
        }
    }
    Ok(argslist.filter())
}

fn traverse<'a>(root: &'a Command, args: &mut Vec<String>) -> Result<&'a Command, String> {
    let mut cmd = root;
    while let Some(arg) = args.first() {
        if arg.starts_with('-') {
            break;
        }
        if let Some(found) = cmd.subcommands.iter().find(|&s| s.name == *arg) {
            cmd = found;
            args.remove(0);
        } else {
            return Err(format!("'{arg}' is not a valid subcommand."));
        }
    }
    Ok(cmd)
}

pub fn parse(root: &Command, input: Vec<String>) -> Result<ParsedCommand, String> {
    let mut input = input;
    input.remove(0);
    let command = traverse(root, &mut input)?;
    Ok(ParsedCommand {
        help: command.help.clone(),
        args: args(command.args.clone(), input)?,
        parents: command.parents.clone(),
    })
}
