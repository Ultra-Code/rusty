use std::env;
use std::fs;
use std::process;

use super::Case;
use crate::mgrep;

#[allow(dead_code)]
struct ArgsRef<'a> {
    query: &'a str,
    file: &'a str,
}

#[allow(dead_code)]
impl<'a> ArgsRef<'a> {
    #[must_use]
    pub fn new<'b>(args: &'b [String]) -> Self
    where
        'b: 'a,
    {
        Self::parse_cli(args)
    }

    fn parse_cli(args: &[String]) -> ArgsRef {
        let args_len = args.len() - 1;
        if args_len < 2 {
            eprintln!(
                r##"
Error: Invalid number of arguments expected 2 got {args_len}
Info: pass the search string and file
Example: cargo run --bin mgrep -- fn src/bin/mgrep.rs
                "##
            );
            process::exit(2);
        };
        let query = &args[1];
        let file = &args[2];

        ArgsRef { query, file }
    }
}

struct Args {
    query: String,
    file: String,
}

impl Args {
    fn new<T>(args: T) -> Result<Self, &'static str>
    where
        T: std::iter::Iterator<Item = String>,
    {
        Self::parse_cli(args)
    }

    fn parse_cli<T>(mut args: T) -> Result<Self, &'static str>
    where
        T: std::iter::Iterator<Item = String>,
    {
        let _program_name = args.next();

        let query = args.next().ok_or("Didn't get a query string")?;

        let file = args.next().ok_or("Didn't get a file path")?;

        Ok(Self { query, file })
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn run(case: Case) {
    let cli_args = Args::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        eprintln!("Example: cargo run --bin mgrep -- fn src/bin/mgrep.rs");
        process::exit(1);
    });
    println!(
        "Searching for `{}` in the file `{}`",
        cli_args.query, cli_args.file
    );

    let content = fs::read_to_string(&cli_args.file).unwrap_or_else(|err| {
        eprintln!("Error: unable to read file {}: {err}", cli_args.file);
        process::exit(1);
    });

    let mgrep_found = mgrep::search(&cli_args.query, &content, case);

    if mgrep_found.is_empty() {
        println!("{} was not found in {}", cli_args.query, cli_args.file);
    } else {
        println!("\nFound `{}` on the following lines:\n", cli_args.query);
        for line in mgrep_found {
            println!("{line}");
        }
    }
}
