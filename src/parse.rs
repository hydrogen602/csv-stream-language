/// Parsing
use crate::eval_chain::Chain;
use crate::namespace::{BuiltinNamespace, NameSpace};
pub use crate::pest::Parser;
use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};

use crate::{
    commands::{Argument, DataTypes},
    rule::MatchPattern,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct IdentParser;

pub fn parse_fail<'i>(e: Error<Rule>) -> Pairs<'i, Rule> {
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

pub fn match_parser(pair: Pair<Rule>) -> MatchPattern {
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

pub fn data_val_parser(pair: Pair<Rule>) -> DataTypes {
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
pub fn arg_parser(pair: Pair<Rule>) -> Argument {
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
pub fn parse_str(s: &str, cmd_line_args: &[&str], namespace: Option<impl NameSpace>) -> Chain {
    let mut pairs = IdentParser::parse(Rule::file, s).unwrap_or_else(parse_fail);

    let flow = pairs.next().unwrap();
    // println!("{:?}", flow);

    let cmds: Box<dyn NameSpace> = match namespace {
        Some(n) => Box::new(n),
        None => Box::new(BuiltinNamespace::default()),
    };

    let mut chain = Chain::default();

    for command in flow.into_inner() {
        let mut parts = command.into_inner();
        let func_name = parts.next().unwrap();
        let args: Vec<_> = parts.map(arg_parser).collect();

        let args = args
            .into_iter()
            .map(|x| Argument::substitute_cmd_line_arg(x, cmd_line_args))
            .collect();

        let f_name = func_name.as_str();

        let cmd = cmds
            .get_command(f_name)
            .expect(&format!("Command {} not found", f_name));

        chain.push(cmd, args);
    }

    //println!("Parse done");

    //chain.execute();
    return chain;
}
