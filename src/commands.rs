use core::fmt;
use std::{collections::HashMap};

use crate::builtins;


#[derive(Debug)]
pub enum Argument {
    String(String),
    Int(i32),
    Float(f64),
    Enum(String),
    Tuple(Vec<Argument>)
}

pub struct Namespace {
    commands: HashMap<String, Command>
}

impl Default for Namespace {
    fn default() -> Self {
        let mut n = Namespace { commands: HashMap::new() };
        let mut helper = |s: &str, f| n.commands.insert(s.into(), f);

        helper("read", builtins::read);
        helper("drop", builtins::drop);
        helper("print", builtins::print);
        helper("columns", builtins::columns);
        helper("write", builtins::write);

        n
    }
}

impl Namespace {
    pub fn get_command(&self, s: &str) -> Option<&Command> {
        self.commands.get(s)
    }
}

// struct FlowIter<T> {

// }

// impl<T> Iterator for FlowIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub enum DataTypes {
    String(String),
    Int(i32),
    Float(f64),
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataTypes::String(s) => s.fmt(f),
            DataTypes::Int(i) => i.fmt(f),
            DataTypes::Float(float) => float.fmt(f)
        }
    }
}

impl From<String> for DataTypes {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for DataTypes {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

impl From<i32> for DataTypes {
    fn from(s: i32) -> Self {
        Self::Int(s)
    }
}

impl From<f64> for DataTypes {
    fn from(s: f64) -> Self {
        Self::Float(s)
    }
}

pub type RowType = Vec<DataTypes>;
pub type GenericIterBox = Box<dyn Iterator<Item=RowType>>;

// pub enum Command {
//     Begin(fn(&Vec<Argument>) -> GenericIterBox),
//     Middle(fn(&Vec<Argument>, GenericIterBox) -> GenericIterBox),
//     End(fn(&Vec<Argument>, GenericIterBox))
// }

pub type Command = fn(&Vec<Argument>, GenericIterBox) -> GenericIterBox;
