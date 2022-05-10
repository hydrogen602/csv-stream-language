extern crate pest;
#[macro_use]
extern crate pest_derive;

mod builtins;
mod commands;
mod eval_chain;
mod namespace;
mod rule;
mod util;

use std::env;
use std::fs::read_to_string;

use commands::{Argument, DataTypes};
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use rule::MatchPattern;

use crate::eval_chain::Chain;
use crate::namespace::Namespace;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

fn parse_fail<'i>(e: Error<Rule>) -> Pairs<'i, Rule> {
    eprintln!("{}", e);
    match e.variant {
        ErrorVariant::ParsingError {
            positives: p,
            negatives: n,
        } => {
            eprintln!("Positives: {:?}", p);
            eprintln!("Negatives: {:?}", n);
        }
        ErrorVariant::CustomError { message: m } => {
            eprintln!("msg: {}", m);
        }
    }
    panic!("");
}

fn match_parser(pair: Pair<Rule>) -> MatchPattern {
    assert_eq!(Rule::pattern, pair.as_rule());

    let actual_rule = pair.into_inner().next().unwrap();
    match actual_rule.as_rule() {
        Rule::string => {
            let s = actual_rule.into_inner().next().unwrap().as_str();
            MatchPattern::compile_regex(s).unwrap()
        }
        r => {
            panic!("Invalid rule: {:?}", r);
        }
    }
}

fn data_val_parser(pair: Pair<Rule>) -> DataTypes {
    match pair.as_rule() {
        Rule::string => {
            //println!("{:?}", pair);
            DataTypes::String(pair.into_inner().next().unwrap().as_str().into())
        }
        Rule::integer => DataTypes::Int(pair.as_str().parse().expect("integer rule wrong")),
        Rule::float => DataTypes::Float(pair.as_str().parse().expect("float rule wrong")),
        r => {
            unreachable!("Got rule {:?}", r);
        }
    }
}

/// Parse a grammar rule into an Argument type
fn arg_parser(pair: Pair<Rule>) -> Argument {
    // println!("{:?}", pair.as_rule());
    // Argument::Int(0)

    let content = pair.as_str();

    match pair.as_rule() {
        Rule::string => {
            //println!("{:?}", pair);
            Argument::String(pair.into_inner().next().unwrap().as_str().into())
        }
        Rule::integer => Argument::Int(content.parse().expect("integer rule wrong")),
        Rule::float => Argument::Float(content.parse().expect("float rule wrong")),
        Rule::ident => Argument::Enum(content.into()),
        Rule::tuple => Argument::Tuple(pair.into_inner().map(arg_parser).collect()),
        Rule::pattern => Argument::Pattern(pair.into_inner().next().map(match_parser).unwrap()),
        Rule::varg => {
            Argument::CmdLineArg(pair.into_inner().next().unwrap().as_str().parse().unwrap())
        }
        Rule::rule => {
            let mut it = pair.into_inner();
            Argument::Rule(
                it.next().map(match_parser).unwrap(),
                it.next().map(data_val_parser).unwrap(),
            )
        }
        r => {
            unreachable!("Got rule {:?}", r);
        }
    }
}

/// Parse the input string and execute it
/// Command line arguments that can be used in the program are given
fn parse_str_and_execute(s: &str, cmd_line_args: &[&str]) {
    let mut pairs = IdentParser::parse(Rule::file, s).unwrap_or_else(parse_fail);

    let flow = pairs.next().unwrap();
    // println!("{:?}", flow);

    let cmds = Namespace::default();

    let mut chain = Chain::default();

    for command in flow.into_inner() {
        let mut parts = command.into_inner();
        let func_name = parts.next().unwrap();
        let mut args: Vec<_> = parts.map(arg_parser).collect();

        let args = args
            .into_iter()
            .map(|x| Argument::substitute_cmd_line_arg(x, cmd_line_args))
            .collect();

        let f_name = func_name.as_str();

        //println!("{:?} {:?}", f_name, args);

        let cmd = cmds
            .get_command(f_name)
            .expect(&format!("Command {} not found", f_name));

        chain.push(cmd, args);
    }

    //println!("Parse done");

    chain.execute();
}

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
