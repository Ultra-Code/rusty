use std::env;
use std::fs;
use std::process;

use super::Case;
use crate::mgrep;

struct Args<'a> {
    query: &'a str,
    file: &'a str,
}

impl<'a> Args<'a> {
    #[must_use]
    pub fn new<'b>(args: &'b [String]) -> Self
    where
        'b: 'a,
    {
        Self::parse_cli(args)
    }

    fn parse_cli(args: &[String]) -> Args {
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

        Args { query, file }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn run(case: Case) {
    let args = env::args().collect::<Vec<String>>();
    let cli_args = Args::new(&args);
    println!(
        "Searching for `{}` in the file `{}`",
        cli_args.query, cli_args.file
    );

    let content = fs::read_to_string(cli_args.file).unwrap_or_else(|err| {
        eprintln!("Error: unable to read file {}: {err}", cli_args.file);
        process::exit(1);
    });

    let mgrep_found = mgrep::search(cli_args.query, &content, case);

    if mgrep_found.is_empty() {
        println!("{} was not found in {}", cli_args.query, cli_args.file);
    } else {
        println!("\nFound `{}` on the following lines:\n", cli_args.query);
        for line in mgrep_found {
            println!("{line}");
        }
    }
}
