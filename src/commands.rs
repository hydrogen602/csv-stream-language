use core::fmt;
use std::{collections::HashMap};


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

        helper("read", Command::Begin(builtins::read));
        helper("drop", Command::Middle(builtins::drop));
        helper("print", Command::End(builtins::print));

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

pub type GenericIterBox = Box<dyn Iterator<Item=Vec<DataTypes>>>;

pub enum Command {
    Begin(fn(&Vec<Argument>) -> GenericIterBox),
    Middle(fn(&Vec<Argument>, GenericIterBox) -> GenericIterBox),
    End(fn(&Vec<Argument>, GenericIterBox))
}

mod builtins {
    // use csv::StringRecord;

    use csv::StringRecord;

    use super::{Argument, GenericIterBox};

    pub fn read(args: &Vec<Argument>) -> GenericIterBox {
        let file = if let [Argument::String(file)] = &args[..] { file } 
        else { 
            unreachable!();
        };

        let reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(file).expect("Could not read file");

        let x = reader.into_records().map(|x| x.expect("Read failed"));

        let it = x.map(|sr: StringRecord| { sr.into_iter().map(|s| s.into()).collect() });

        Box::new(it)
    }

    // pub enum DropIterator<I: Iterator> {
    //     HeadDrop(I),
    //     //TailDrop(I, queues::CircularBuffer<>)
    // }

    // impl<I: Iterator> Iterator for DropIterator<I> {
    //     type Item = I::Item;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         match self {
    //             Self::HeadDrop(it) => { it.next() }
    //         }
    //     }
    // }

    // impl<I: Iterator> DropIterator<I> {
    //     fn head(it: GenericIterBox, skip_rows: usize) -> Self {
    //         for _ in 0..skip_rows {
    //             if let None = it.next() {
    //                 return Self::HeadDrop(iter::empty());
    //             }
    //         }
    //         Self::HeadDrop(*it)
    //     }
    // }

    pub fn drop(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
        if let [Argument::Enum(op), Argument::Int(n)] = &args[..] {
            match op.as_str() {
                "head" => {
                    let x = input.skip(*n as usize);
                    // for _ in 0..*n {
                    //     if let None = input.next() {
                    //         break;
                    //     }
                    // }
                    Box::new(x) //input
                },
                "tail" => {
                    todo!("tail not yet supported");
                },
                arg => panic!("Invalid argument for drop: {}", arg)
            }
        } else {
            unreachable!();
        }
    }

    pub fn print(args: &Vec<Argument>, input: GenericIterBox) {
        if args.len() > 0 {
            panic!( "Invalid arguments: {:?}" , args );
        }

        for row in input {
            print!("[");
            for (i, elem) in row.into_iter().enumerate() {
                if i == 0 {
                    print!("{}", elem);
                }
                else {
                    print!(", {}", elem);
                }
            }
            println!("]");
        }
    }
}


// pub mod builtins {
//     use std::fs::read_to_string;

//     use super::{Command, Argument};
//     pub struct Read { path: String }
    

//     impl Read {
//         pub fn new(args: Vec<Argument>) -> Box<dyn Command> {
//             
//         }
//     }

//     enum DropOptions { head, tail }
//     pub struct DropCommand { side: DropOptions, count: i32 }
//     impl Command for DropCommand {}

//     impl DropCommand {
//         pub fn new(args: Vec<Argument>) -> Box<dyn Command> {
//             Box::new(match &args[..] {
//                 [Argument::Enum(op), Argument::Int(n)] => { 
//                     DropCommand { side: match op.as_str() {
//                         "head" => DropOptions::head,
//                         "tail" => DropOptions::tail,
//                         arg => panic!("Invalid argument for drop: {}", arg)
//                     }, count: *n } 
//                 }
//                 _ => unreachable!()
//             })
//         }
//     }
// }
