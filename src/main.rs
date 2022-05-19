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

use parse::parse_str_and_execute;
use pest::error::Error;
use pest::iterators::Pairs;

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
            let s = read_to_string(file).expect(&format!("Could not read file: {}", file));

            parse_str_and_execute(&s, cmd_line_args);
        }
    }
}
