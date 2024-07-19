use crate::*;

fn license(cmd: &CommandBuilder<String>) -> String {
    if let Some(license) = &cmd.license {
        format!("Licensed under {license}")
    } else {
        "".into()
    }
}

fn subcommands(cmd: &CommandBuilder<String>) -> String {
    if cmd.subcommands.is_empty() {
        return "".into();
    }
    let mut buf = String::from("SUBCOMMANDS:\n");
    for subcmd in &cmd.subcommands {
        buf.push_str(&format!(
            "\t{name}{tabs}{description}\n",
            name = subcmd.name,
            description = subcmd.description,
            tabs = tabs(subcmd.name.len())
        ));
    }
    buf
}

fn tabs(len: usize) -> &'static str {
    match len / 8 {
        0 => "\t\t\t",
        1 => "\t\t",
        2 => "\t",
        _ => "\n\t\t\t",
    }
}

fn args(cmd: &CommandBuilder<String>) -> String {
    if cmd.args.is_empty() {
        return "".into();
    }
    let mut buf = String::from("ARGS:\n");
    for arg in &cmd.args.args {
        let name = arg.argname.to_string();
        buf.push_str(&format!(
            "\t{name}{tabs}{description}\n",
            description = arg.description,
            tabs = tabs(name.len())
        ));
    }
    buf
}

fn usage(cmd: &CommandBuilder<String>, fullname: &str) -> String {
    let mut buf = String::from("USAGE:");
    if !cmd.args.is_empty() {
        buf.push_str(&format!("\n\t{fullname} [ARGS]"))
    }
    if !cmd.subcommands.is_empty() {
        buf.push_str(&format!("\n\t{fullname} [SUBCOMMAND] [ARGS]"))
    }
    buf
}

pub fn create(cmd: &CommandBuilder<String>) -> String {
    let fullname = format!("{} {}", cmd.parents.join(" "), cmd.name);
    let fullname = fullname.trim();
    format!(
        "{fullname} {version}
{author}
{description}

{usage}

{args}
{subcommands}
{license}",
        description = cmd.description,
        version = cmd.version.clone().unwrap_or("".into()),
        author = cmd.author.clone().unwrap_or("".into()),
        usage = usage(cmd, fullname),
        args = args(cmd),
        subcommands = subcommands(cmd),
        license = license(cmd)
    )
}
