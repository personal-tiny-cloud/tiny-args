# What is this?

This is a bare-bones parser for CLI commands made for Tiny Cloud.
It was made in place of [clap](https://docs.rs/clap/latest/clap/) because
tcloud plugins need a way to configure and execute subcommands from different crates.
It can be used for other projects too, but keep it mind that it is made for a specific project,
so if you need some particular features that are not supposed to be here you should use some other crate.

## Example

```rust
use tiny_args::*;

let parsed = Command::create("myapp", "This is my cool app!")
       .arg(arg!(-h, --help), ArgType::Flag, "Shows help.")
       .arg(arg!(-V), ArgType::Flag, "Shows this project's version.")
       .arg(arg!(--path), ArgType::Path, "Path to something.")
       .author("Me!")
       .build()
       .parse()
       .unwrap(); // It would be better to show the error to the user instead of panicking

if parsed.args.get(arg!(-h)).is_some() {
    println!("{}", parsed.help);
    return;
}

if let Some(path) = parsed.args.get(arg!(--path)) {
    let mut pathbuf = path.value().path();
    pathbuf.push("some/other/path");
    println!("My path: {path}", path = pathbuf.into_os_string().into_string().unwrap());
}
```

# Docs

Since this crate is not on [crates.io](https://crates.io/) ((yet)) there is no online documentation.

But, you can still build it locally:

- Install [rustup](https://rustup.rs/) if you haven't done it already;
- Clone the repo;
- Run `cargo doc` on the repo;
- The path to the html page should appear:
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
   Generated /path/to/tiny-args/target/doc/tiny_args/index.html
```
- Copy-paste the path into your browser's search bar.

# License

This project is licensed under the [GNU General Public License 3.0](/LICENSE)
