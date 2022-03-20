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
    commands: HashMap<String, fn(Vec<Argument>) -> Box<dyn Command>>
}

impl Default for Namespace {
    fn default() -> Self {
        use builtins::*;
        let mut n = Namespace { commands: HashMap::new() };
        let mut helper = |s: &str, f| n.commands.insert(s.into(), f);

        helper("read", Read::new);
        helper("drop", DropCommand::new);

        n
    }
}

impl Namespace {
    fn get_command(&self, name: &str, args: Vec<Argument>) -> Box<dyn Command> {
        let f = self.commands.get(name).expect(format!("command not found: {}", name).as_str());
        f(args)
        
    }
}


pub trait Command {
    //fn run<I: Iterator>(&self) -> I;
    fn run(&self) -> Iterator {}
}

pub mod builtins {
    use std::fs::read_to_string;

    use super::{Command, Argument};
    pub struct Read { path: String }
    impl Command for Read {
        fn run(&self) -> impl Iterator {
            let x = csv::ReaderBuilder::new().has_headers(false).from_path(&self.path).expect("Could not read file");
            x.records()
        }
    }

    impl Read {
        pub fn new(args: Vec<Argument>) -> Box<dyn Command> {
            match &args[..] {
                [Argument::String(file)] => { Box::new(Read { path: file.clone() }) }
                _ => unreachable!()
            }
        }
    }

    enum DropOptions { head, tail }
    pub struct DropCommand { side: DropOptions, count: i32 }
    impl Command for DropCommand {}

    impl DropCommand {
        pub fn new(args: Vec<Argument>) -> Box<dyn Command> {
            Box::new(match &args[..] {
                [Argument::Enum(op), Argument::Int(n)] => { 
                    DropCommand { side: match op.as_str() {
                        "head" => DropOptions::head,
                        "tail" => DropOptions::tail,
                        arg => panic!("Invalid argument for drop: {}", arg)
                    }, count: *n } 
                }
                _ => unreachable!()
            })
        }
    }
}
