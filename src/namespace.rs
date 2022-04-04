use core::fmt;
use std::{collections::HashMap, error::Error};

use crate::{commands::Command, builtins};


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
        helper("parse", builtins::parse);
        helper("classify", builtins::classify);
        helper("filter", builtins::filter);

        // summary funcs
        helper("sum", builtins::summary::sum);

        n
    }
}


#[derive(Debug, Clone)]
pub struct CommandExistsError;

impl fmt::Display for CommandExistsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command exists already")
    }
}

impl Error for CommandExistsError {}

impl Namespace {
    pub fn get_command(&self, s: &str) -> Option<&Command> {
        self.commands.get(s)
    }

    #[allow(dead_code)]
    pub fn add_command(&mut self, s: &str, cmd: Command) -> Result<(), CommandExistsError> {
        if self.commands.contains_key(s) {
            Err(CommandExistsError)
        }
        else {
            self.commands.insert(s.into(), cmd);
            Ok(())
        }
    }
}
