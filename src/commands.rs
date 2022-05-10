use core::fmt;
use std::{ops::Add, str::FromStr};

use crate::{
    arg_parser, pest::Parser, rule::MatchPattern, Error, ErrorVariant, IdentParser, Pairs, Rule,
};
use chrono::NaiveDate;

#[derive(Debug)]
pub enum Argument {
    String(String),
    Int(i32),
    Float(f64),
    Enum(String),
    Rule(MatchPattern, DataTypes),
    Pattern(MatchPattern),
    Tuple(Vec<Argument>),
    CmdLineArg(u32), // u32 is command line arg number, like 1 for $1
}

impl Argument {
    pub fn substitute_cmd_line_arg(self: Self, cmd_line_args: &[&str]) -> Self {
        match self {
            Self::CmdLineArg(num) => {
                if num >= cmd_line_args.len() as u32 {
                    panic!(
                        "Invalid command line argument: Expected ${num}, but only {} are supplied",
                        cmd_line_args.len()
                    );
                }

                let value = &cmd_line_args[num as usize];

                let parse_fail = |e: Error<Rule>| -> Pairs<Rule> {
                    eprintln!("{}", e);
                    panic!("Cannot parse command line argument: {}", value);
                };

                let mut pairs = IdentParser::parse(Rule::expr, value).unwrap_or_else(parse_fail);
                arg_parser(pairs.next().unwrap())
            }
            other => other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypes {
    Empty,
    String(String),
    Int(i32),
    Float(f64),
    NaiveDate(NaiveDate),
}

impl Default for DataTypes {
    fn default() -> Self {
        DataTypes::Empty
    }
}

impl FromStr for DataTypes {
    type Err = (); // should never error

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // most restrictive to least
        // TODO: dates

        if s.is_empty() {
            Ok(DataTypes::Empty)
        } else if let Ok(n) = s.parse::<i32>() {
            Ok(DataTypes::Int(n))
        } else if let Ok(f) = s.parse::<f64>() {
            Ok(DataTypes::Float(f))
        } else {
            Ok(DataTypes::String(s.into()))
        }
    }
}

impl From<DataTypes> for String {
    fn from(d: DataTypes) -> Self {
        match d {
            DataTypes::Empty => "".to_string(),
            DataTypes::String(s) => s,
            DataTypes::Int(i) => i.to_string(),
            DataTypes::Float(f) => f.to_string(),
            DataTypes::NaiveDate(date) => date.to_string(),
        }
    }
}

impl From<DataTypes> for i32 {
    fn from(d: DataTypes) -> Self {
        match d {
            DataTypes::Empty => 0,
            DataTypes::String(s) => s.parse().expect(&format!("could not parse {} as int", s)),
            DataTypes::Int(i) => i,
            DataTypes::Float(f) => f as i32,
            DataTypes::NaiveDate(_) => panic!("could not parse date as int"),
        }
    }
}

impl From<DataTypes> for f64 {
    fn from(d: DataTypes) -> Self {
        match d {
            DataTypes::Empty => 0.,
            DataTypes::String(s) => s.parse().expect(&format!("could not parse {} as float", s)),
            DataTypes::Int(i) => i as f64,
            DataTypes::Float(f) => f,
            DataTypes::NaiveDate(_) => panic!("could not parse date as float"),
        }
    }
}

impl From<DataTypes> for NaiveDate {
    fn from(d: DataTypes) -> Self {
        match d {
            // TODO: get rid of panics
            DataTypes::String(s) => NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .expect(&format!("Could not parse {} as date", s)),
            DataTypes::NaiveDate(d) => d,
            _ => panic!(),
        }
    }
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataTypes::Empty => "".fmt(f),
            DataTypes::String(s) => s.fmt(f),
            DataTypes::Int(i) => i.fmt(f),
            DataTypes::Float(float) => float.fmt(f),
            DataTypes::NaiveDate(date) => date.fmt(f),
        }
    }
}

macro_rules! from_impl {
    ($enum_name:ident, $ty:ty) => {
        impl From<$ty> for DataTypes {
            fn from(s: $ty) -> Self {
                Self::$enum_name(s.into())
            }
        }
    };
}

// handled by FromStr
// impl From<String> for DataTypes {
//     fn from(s: String) -> Self {
//         if s.is_empty() {
//             Self::Empty
//         }
//         else {
//             Self::String(s.into())
//         }
//     }
// }

from_impl!(String, String);
from_impl!(String, &str);
from_impl!(Int, i32);
from_impl!(Float, f64);
from_impl!(NaiveDate, NaiveDate);

impl Add for DataTypes {
    type Output = DataTypes;

    fn add(self, rhs: Self) -> Self::Output {
        use DataTypes::*;

        match (self, rhs) {
            (Empty, Empty) => Empty,
            (Empty, rhs) => rhs,
            (lhs, Empty) => lhs,
            (String(mut s), rhs) => {
                let r: std::string::String = rhs.into();
                s.push_str(&r);
                String(s)
            }
            (lhs, String(s)) => {
                let mut l: std::string::String = lhs.into();
                l.push_str(&s);
                String(l)
            }
            (Float(f), Int(rn)) => Float(f + rn as f64),
            (Float(f), Float(rn)) => Float(f + rn as f64),
            (Int(ln), Float(rn)) => Float(ln as f64 + rn),
            (Int(ln), Int(rn)) => Int(ln + rn),
            (NaiveDate(_), _) => panic!("Cannot sum dates"),
            (_, NaiveDate(_)) => panic!("Cannot sum dates"),
        }
    }
}

pub type RowType = Vec<DataTypes>;
pub type GenericIterBox = Box<dyn Iterator<Item = RowType>>;

pub type Command = fn(Vec<Argument>, GenericIterBox) -> GenericIterBox;
