use core::fmt;

use chrono::NaiveDate;
use crate::rule::MatchPattern;

#[derive(Debug)]
pub enum Argument {
    String(String),
    Int(i32),
    Float(f64),
    Enum(String),
    Rule(MatchPattern, DataTypes),
    Pattern(MatchPattern),
    Tuple(Vec<Argument>)
}


// impl TryFrom<Argument> for DataTypes {
//     type Error = ;

//     fn try_from(value: Argument) -> Result<Self, Self::Error> {
        
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypes {
    String(String),
    Int(i32),
    Float(f64),
    NaiveDate(NaiveDate)
}

impl From<DataTypes> for String {
    fn from(d: DataTypes) -> Self {
        match d {
            DataTypes::String(s) => s,
            DataTypes::Int(i) => i.to_string(),
            DataTypes::Float(f) => f.to_string(),
            DataTypes::NaiveDate(date) => date.to_string()
        }
    }
}

impl From<DataTypes> for i32 {
    fn from(d: DataTypes) -> Self {
        match d {
            DataTypes::String(s) => s.parse().expect(&format!("could not parse {} as int", s)),
            DataTypes::Int(i) => i,
            DataTypes::Float(f) => f as i32,
            DataTypes::NaiveDate(_) => panic!("could not parse date as int")
        }
    }
}

impl From<DataTypes> for f64 {
    fn from(d: DataTypes) -> Self {
        match d {
            DataTypes::String(s) => s.parse().expect(&format!("could not parse {} as float", s)),
            DataTypes::Int(i) => i as f64,
            DataTypes::Float(f) => f,
            DataTypes::NaiveDate(_) => panic!("could not parse date as float")
        }
    }
}

impl From<DataTypes> for NaiveDate {
    fn from(d: DataTypes) -> Self {
        match d {
            // TODO: get rid of panics
            DataTypes::String(s) => 
                NaiveDate::parse_from_str(&s, "%Y-%m-%d").expect(
                    &format!("Could not parse {} as date", s)),
            DataTypes::NaiveDate(d) => d,
            _ => panic!()
        }
    }
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataTypes::String(s) => s.fmt(f),
            DataTypes::Int(i) => i.fmt(f),
            DataTypes::Float(float) => float.fmt(f),
            DataTypes::NaiveDate(date) => date.fmt(f)
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

from_impl!(String, String);
from_impl!(String, &str);
from_impl!(Int, i32);
from_impl!(Float, f64);
from_impl!(NaiveDate, NaiveDate);


pub type RowType = Vec<DataTypes>;
pub type GenericIterBox = Box<dyn Iterator<Item=RowType>>;

pub type Command = fn(Vec<Argument>, GenericIterBox) -> GenericIterBox;
