extern crate pest;
#[macro_use]
extern crate pest_derive;

mod commands;
mod eval_chain;
mod builtins;
mod namespace;
mod util;
mod rule;

use std::fs::read_to_string;

use commands::{Argument, DataTypes};
use rule::MatchPattern;
use pest::Parser;
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pairs, Pair};

use crate::namespace::Namespace;
use crate::eval_chain::Chain;


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

fn parse_fail<'i>(e: Error<Rule>) -> Pairs<'i, Rule> {
    eprintln!("{}", e);
    match e.variant {
        ErrorVariant::ParsingError { positives: p, negatives: n } => {
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
    let s = pair.into_inner().next().unwrap().as_str();

    MatchPattern::compile_regex(s).unwrap()
}

fn data_val_parser(pair: Pair<Rule>) -> DataTypes {
    match pair.as_rule() {
        Rule::string => {
            //println!("{:?}", pair);
            DataTypes::String(pair.into_inner().next().unwrap().as_str().into())
        },
        Rule::integer => DataTypes::Int(pair.as_str().parse().expect("integer rule wrong")),
        Rule::float => DataTypes::Float(pair.as_str().parse().expect("float rule wrong")),
        r => {
            unreachable!("Got rule {:?}", r);
        }
    }
}

fn arg_parser(pair: Pair<Rule>) -> Argument {
    // println!("{:?}", pair.as_rule());
    // Argument::Int(0)

    let content = pair.as_str();

    match pair.as_rule() {
        Rule::string => {
            //println!("{:?}", pair);
            Argument::String(pair.into_inner().next().unwrap().as_str().into())
        },
        Rule::integer => Argument::Int(content.parse().expect("integer rule wrong")),
        Rule::float => Argument::Float(content.parse().expect("float rule wrong")),
        Rule::ident => Argument::Enum(content.into()),
        Rule::tuple => Argument::Tuple(pair.into_inner().map(arg_parser).collect()),
        Rule::rule => {
            let mut it = pair.into_inner();
            Argument::Rule(it.next().map(match_parser).unwrap(), it.next().map(data_val_parser).unwrap())
        },
        r => {
            unreachable!("Got rule {:?}", r);
        }
    }
}

fn main() {
    let s = read_to_string("test.fluss").expect("yeet");

    let mut pairs = IdentParser::parse(Rule::file, &s).unwrap_or_else(parse_fail);

    let flow = pairs.next().unwrap();
    // println!("{:?}", flow);

    let cmds = Namespace::default();

    let mut chain = Chain::default();

    for command in flow.into_inner() {
        let mut parts = command.into_inner();
        let func_name = parts.next().unwrap();
        let args: Vec<_> = parts.map(arg_parser).collect();

        let f_name = func_name.as_str();

        //println!("{:?} {:?}", f_name, args);

        let cmd = cmds.get_command(f_name).expect(&format!("Command {} not found", f_name));

        chain.push(cmd,args);
    }

    //println!("Parse done");

    chain.execute();
}
