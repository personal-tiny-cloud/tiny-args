#![crate_name = "tiny_args"]
//#![warn(missing_docs)]

//! # What is this?
//!
//! This is a bare-bones parser for CLI commands made for Tiny Cloud.
//! It was made in place of [clap](https://docs.rs/clap/latest/clap/) because
//! tcloud plugins need a way to configure and execute subcommands from different crates.
//! It can be used for other projects too, but keep it mind that it is made for a specific project,
//! so if you need some particular features that are not supposed to be here you should use some other crate.
//!
//! # Example
//!
//! ```rust
//! use tiny_args::*;
//!
//! let parsed = Command::create("myapp", "This is my cool app!")
//!        .arg(arg!(-h, --help), ArgType::Flag, "Shows help.")
//!        .arg(arg!(-V), ArgType::Flag, "Shows this project's version.")
//!        .arg(arg!(--path), ArgType::Path, "Path to something.")
//!        .author("Me!")
//!        .build()
//!        .parse()
//!        .unwrap(); // It would be better to show the error to the user instead of panicking
//!
//! if parsed.args.get(arg!(-h)).is_some() {
//!     println!("{}", parsed.help);
//!     return;
//! }
//!
//! if let Some(path) = parsed.args.get(arg!(--path)) {
//!     let mut pathbuf = path.value().path();
//!     pathbuf.push("some/other/path");
//!     println!("My path: {path}", path = pathbuf.into_os_string().into_string().unwrap());
//! }
//! ```

use std::{env, fmt, path::PathBuf};

mod help;
mod parser;
mod tests;
#[macro_use]
mod macros;

/// Arguments' value types.
/// Used when configuring the command line to specify what values each argument contains.
#[derive(Clone, Debug)]
pub enum ArgType {
    /// A [`String`].
    String,

    /// An [`i64`].
    Num,

    /// A [`f64`].
    Float,

    /// A [`PathBuf`]
    Path,

    /// A normal flag, does not contain any value.
    Flag,
}

/// Arguments' values.
/// Accessible after the command line has been parsed with [`Command::parse`] or [`Command::parse_from`].
#[derive(Clone, Debug)]
pub enum ArgValue {
    /// See [`ArgValue::string`]
    String(String),

    /// See [`ArgValue::num`]
    Num(i64),

    /// See [`ArgValue::float`]
    Float(f64),

    /// See [`ArgValue::path`]
    Path(PathBuf),

    /// Flags do not carry any value.
    /// You can see if they were activated by checking if the argument exists in the argument's
    /// list after the command line has been parsed.
    Flag,
}

impl ArgValue {
    /// Unwraps the value and returns the inner string.
    ///
    /// # Panic
    ///
    /// It panics if the value is not a string.
    pub fn string(self) -> String {
        match self {
            Self::String(s) => s,
            _ => panic!("This argument's value is not a string"),
        }
    }

    /// Unwraps the value and returns the inner [`i64`].
    ///
    /// # Panic
    ///
    /// It panics if the value is not a number.
    pub fn num(self) -> i64 {
        match self {
            Self::Num(n) => n,
            _ => panic!("This argument's value is not a number"),
        }
    }

    /// Unwraps the value and returns the inner [`f64`].
    ///
    /// # Panic
    ///
    /// It panics if the value is not a float.
    pub fn float(self) -> f64 {
        match self {
            Self::Float(f) => f,
            _ => panic!("This argument's value is not a float"),
        }
    }

    /// Unwraps the value and returns the inner [`PathBuf`].
    ///
    /// # Panic
    ///
    /// It panics if the value is not a path.
    pub fn path(self) -> PathBuf {
        match self {
            Self::Path(p) => p,
            _ => panic!("This argument's value is not a path"),
        }
    }
}

/// Name of an argument. It contains both short and/or long version of the argument.
///
/// You can either use this enum or its shorthand macro [`arg`]. This macro contains both short
/// and/or long versions of the argument. Two arguments with the same short and/or long version
/// will be treated as equal (for example `ArgName::Both { short: 'h', long: "help" } ==
/// ArgName::Short('h')` is `true`). When initializing a [`Command`] with the [`CommandBuilder`] the
/// generic type can be anything, but it will be turned into [`String`] once built with [`CommandBuilder::build`]
/// or when turned into a [`SubCommand`].
#[derive(Eq, Clone, Debug)]
pub enum ArgName<T: Into<String> + Clone + Eq> {
    /// Represents a short argument.
    ///
    /// It is formed by a dash and a character on the command line (e.g. `-h`).
    /// The dash is omitted in the enum's value.
    /// When turned into a string this enum recreates the argument.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tiny_args::ArgName;
    /// assert_eq!(ArgName::Short::<String>('h').to_string(), "-h");
    /// ```
    Short(char),

    /// Represents a long argument.
    ///
    /// It is formed by two dashes and a string on the command line (e.g. `--help`).
    /// The dashes are omitted in the enum's value.
    /// When turned into a string this enum recreates the argument.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tiny_args::ArgName;
    /// assert_eq!(ArgName::Long("help").to_string(), "--help");
    /// ```
    Long(T),

    /// Represents both a long and a short argument.
    ///
    /// When turned into a string this enum prints both arguments.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tiny_args::ArgName;
    /// assert_eq!(ArgName::Both { short: 'h', long: "help" }.to_string(), "-h, --help");
    /// ```
    Both { short: char, long: T },
}

impl<T: Into<String> + Clone + Eq> ArgName<T> {
    fn into_string(self) -> ArgName<String> {
        match self {
            Self::Short(c) => ArgName::Short(c),
            Self::Long(s) => ArgName::Long(s.into()),
            Self::Both { short, long } => ArgName::Both {
                short,
                long: long.into(),
            },
        }
    }
}

impl<T: Into<String> + Clone + Eq> PartialEq for ArgName<T> {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Self::Short(s) => match *other {
                Self::Short(o) => *s == o,
                Self::Long(_) => false,
                Self::Both { short, .. } => *s == short,
            },
            Self::Long(s) => match &other {
                Self::Short(_) => false,
                Self::Long(o) => *s == *o,
                Self::Both { long, .. } => *s == *long,
            },
            Self::Both { short, long } => match &other {
                Self::Short(o) => *short == *o,
                Self::Long(o) => *long == *o,
                Self::Both { short: s, long: l } => *short == *s || *long == *l,
            },
        }
    }
}

impl<T: Into<String> + Clone + Eq> fmt::Display for ArgName<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Short(s) => write!(f, "-{s}"),
            Self::Long(l) => write!(f, "--{}", Into::<String>::into(l.clone())),
            Self::Both { short, long } => {
                write!(f, "-{short}, --{}", Into::<String>::into(long.clone()))
            }
        }
    }
}

/// A struct containing all the information of an argument.
///
/// This struct should not be available until [`Command`] has been parsed.
#[derive(Clone)]
pub struct Arg<T: Into<String> + Clone + Eq> {
    argname: ArgName<T>,
    argtype: ArgType,
    argvalue: Option<ArgValue>,
    description: T,
}

impl<T: Into<String> + Clone + Eq> Arg<T> {
    fn new(argname: ArgName<T>, argtype: ArgType, description: T) -> Self {
        Self {
            argname,
            argtype,
            argvalue: None,
            description,
        }
    }
    
    /// [`ArgType`] of this argument.
    pub fn argtype(&self) -> ArgType {
        self.argtype.clone()
    }
    
    /// [`ArgValue`] of this argument.
    ///
    /// # Panic
    ///
    /// This function panics if the value is [`None`], but this should never happen since
    /// [`Arg`] is available only after [`Command`] has been parsed.
    pub fn value(&self) -> ArgValue {
        self.argvalue
            .clone()
            .expect("Tried to access the value of an uninitialized argument.")
    }
    
    /// Return the description of this argument.
    pub fn description(&self) -> String {
        self.description.clone().into()
    }

    fn into(self) -> Arg<String> {
        Arg {
            argname: self.argname.into_string(),
            argtype: self.argtype,
            argvalue: None,
            description: self.description.into(),
        }
    }
}

impl Arg<String> {
    fn init(&mut self, argvalue: ArgValue) {
        self.argvalue = Some(argvalue);
    }
}

#[derive(Clone)]
pub struct ArgList<T: Into<String> + Clone + Eq> {
    args: Vec<Arg<T>>,
}

impl<T: Into<String> + Clone + Eq> ArgList<T> {
    fn new() -> Self {
        ArgList { args: Vec::new() }
    }

    fn insert(&mut self, arg: Arg<T>) {
        if self.args.iter().any(|a| a.argname == arg.argname) {
            panic!("{} already exists", arg.argname);
        }
        self.args.push(arg);
    }

    fn filter(self) -> Self {
        ArgList {
            args: self
                .args
                .into_iter()
                .filter(|a| a.argvalue.is_some())
                .collect(),
        }
    }

    fn into(self) -> ArgList<String> {
        ArgList {
            args: self.args.into_iter().map(|a| a.into()).collect(),
        }
    }
}

impl ArgList<String> {
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn get(&self, argname: ArgName<&str>) -> Option<Arg<String>> {
        let argname = argname.into_string();
        for arg in &self.args {
            if arg.argname == argname {
                return Some(arg.clone());
            }
        }
        None
    }

    fn init_arg(&mut self, argname: &ArgName<String>, argvalue: ArgValue) {
        for arg in &mut self.args {
            if arg.argname == *argname {
                arg.init(argvalue);
                return;
            }
        }
        panic!("Tried to initialize {argname} but it does not exist");
    }
}

pub struct CommandBuilder<T: Into<String> + Clone + Eq> {
    name: T,
    description: T,
    version: Option<T>,
    author: Option<T>,
    license: Option<T>,
    args: ArgList<T>,
    subcommands: Vec<Command>,
    parents: Vec<String>,
}

pub type SubCommand = CommandBuilder<String>;

impl<T: Into<String> + Clone + Eq> CommandBuilder<T> {
    pub fn arg(mut self, argname: ArgName<T>, argtype: ArgType, description: T) -> Self {
        self.args.insert(Arg::new(argname, argtype, description));
        self
    }

    pub fn subcommand(mut self, subcmd: SubCommand) -> Self {
        if self.subcommands.iter().any(|s| s.name == subcmd.name) {
            panic!("Subcommand '{}' already exists.", subcmd.name);
        }
        let mut subcmd = subcmd;
        subcmd.add_parents(self.parents.clone(), self.name.clone().into());
        self.subcommands.push(subcmd.build());
        self
    }

    pub fn version(mut self, version: T) -> Self {
        self.version = Some(version);
        self
    }

    pub fn author(mut self, author: T) -> Self {
        self.author = Some(author);
        self
    }

    pub fn license(mut self, license: T) -> Self {
        self.license = Some(license);
        self
    }

    fn add_parents(&mut self, grandparents: Vec<String>, parent: String) {
        let mut parents = grandparents;
        parents.push(parent);
        self.parents = parents;
    }

    pub fn into(self) -> SubCommand {
        CommandBuilder {
            name: self.name.into(),
            description: self.description.into(),
            version: self.version.map(|v| v.into()),
            author: self.author.map(|a| a.into()),
            license: self.license.map(|l| l.into()),
            args: self.args.into(),
            subcommands: self.subcommands,
            parents: self.parents,
        }
    }

    pub fn build(self) -> Command {
        let cmd = self.into();
        Command {
            help: help::create(&cmd),
            name: cmd.name,
            description: cmd.description,
            args: cmd.args,
            subcommands: cmd.subcommands,
            parents: cmd.parents,
        }
    }
}

pub struct Command {
    pub help: String,
    name: String,
    description: String,
    args: ArgList<String>,
    subcommands: Vec<Command>,
    parents: Vec<String>,
}

impl Command {
    pub fn create<T: Into<String> + Clone + Eq>(name: T, description: T) -> CommandBuilder<T> {
        CommandBuilder {
            name,
            description,
            version: None,
            author: None,
            license: None,
            args: ArgList::new(),
            subcommands: Vec::new(),
            parents: Vec::new(),
        }
    }

    pub fn parse(&self) -> Result<ParsedCommand, String> {
        self.parse_from(env::args().collect())
    }

    pub fn parse_from(&self, args: Vec<String>) -> Result<ParsedCommand, String> {
        parser::parse(self, args)
    }
}

#[non_exhaustive]
pub struct ParsedCommand {
    pub help: String,
    pub args: ArgList<String>,
    pub parents: Vec<String>,
}
