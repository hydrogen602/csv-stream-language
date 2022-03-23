extern crate pest;
#[macro_use]
extern crate pest_derive;

mod commands;
mod eval_chain;
mod builtins;

use std::fs::read_to_string;

use commands::Argument;
use pest::Parser;
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pairs, Pair};

use crate::commands::Namespace;
use crate::eval_chain::Chain;


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

fn parse_fail<'i>(e: Error<Rule>) -> Pairs<'i, Rule> {
    eprintln!("{}", e);
    panic!("");
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
