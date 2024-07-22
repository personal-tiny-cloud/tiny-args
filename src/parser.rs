use crate::*;

fn args(
    mut argslist: ArgList<String>,
    mut inputargs: Vec<String>,
) -> Result<ArgList<String>, String> {
    let mut argnameopt: Option<ArgName<String>> = None;
    while let Some(input) = inputargs.first() {
        if let Some(argname) = &argnameopt {
            argslist.init_arg(argname, &mut inputargs)?;
            argnameopt = None;
        } else if input.starts_with("--") {
            if let Some(input) = input.get(2..) {
                argnameopt = Some(ArgName::Long(input.into()));
                inputargs.remove(0);
            } else {
                return Err(format!("'{input}' is not a valid long argument."));
            }
        } else if input.starts_with('-') {
            if let Some(input) = input.chars().nth(1) {
                argnameopt = Some(ArgName::Short(input));
                inputargs.remove(0);
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
