extern crate pest;
#[macro_use]
extern crate pest_derive;

mod builtins;
mod commands;
mod eval_chain;
mod namespace;
mod parse;
mod rule;
mod util;

use std::env;
use std::fs::read_to_string;

use namespace::BuiltinNamespace;
use parse::parse_str;

fn main() {
    let args: Vec<String> = env::args().collect();

    let b_args: Vec<&str> = args.iter().map(String::as_str).collect();

    match b_args[..] {
        [] => {
            unreachable!();
        }
        [exec] => {
            eprintln!("Usage: {exec} file")
        }
        [_, ref cmd_line_args @ ..] => {
            let file = cmd_line_args[0];
            let file_content =
                read_to_string(file).expect(&format!("Could not read file: {}", file));

            let chain = parse_str(&file_content, cmd_line_args, None::<BuiltinNamespace>);

            chain.execute();
        }
    }
}
