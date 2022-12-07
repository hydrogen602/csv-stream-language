/// Definition of Namespace, i.e. the mapping of strings to functions (Commands)
use core::fmt;
use std::{collections::HashMap, error::Error};

use crate::{builtins, commands::Command};

pub trait NameSpace {
    fn get_command(&self, s: &str) -> Option<Command>;
}

pub struct BuiltinNamespace {
    commands: HashMap<String, Command>,
}

impl Default for BuiltinNamespace {
    fn default() -> Self {
        let mut n = BuiltinNamespace {
            commands: HashMap::new(),
        };
        let mut helper = |s: &str, f| n.commands.insert(s.into(), f);

        #[cfg(not(target_family = "wasm"))]
        helper("read", builtins::read);

        #[cfg(target_family = "wasm")]
        helper("read", builtins::read_in);

        helper("drop", builtins::drop);
        helper("print", builtins::print);
        helper("columns", builtins::columns);
        helper("write", builtins::write);
        helper("parse", builtins::parse);
        helper("classify", builtins::classify);
        helper("filter", builtins::filter);
        helper("range", builtins::range);

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

impl NameSpace for BuiltinNamespace {
    fn get_command(&self, s: &str) -> Option<Command> {
        self.commands.get(s).copied()
    }
}

impl BuiltinNamespace {
    #[allow(dead_code)]
    pub fn add_command(&mut self, name: &str, cmd: Command) -> Result<(), CommandExistsError> {
        if self.commands.contains_key(name) {
            Err(CommandExistsError)
        } else {
            self.commands.insert(name.into(), cmd);
            Ok(())
        }
    }
}
