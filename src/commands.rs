use core::fmt;

#[derive(Debug)]
pub enum Argument {
    String(String),
    Int(i32),
    Float(f64),
    Enum(String),
    Tuple(Vec<Argument>)
}

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

pub type Command = fn(&Vec<Argument>, GenericIterBox) -> GenericIterBox;
