use std::iter::Empty;
use crate::commands::{Command, Argument, GenericIterBox};

pub struct Chain<'a> {
    chain: Vec<(&'a Command, Vec<Argument>)>
}

impl<'a> Chain<'a> {
    pub fn push(&mut self, cmd: &'a Command, args: Vec<Argument>) {
        self.chain.push((cmd, args));
    }
}

impl Chain<'_> {
    pub fn execute(&self) -> usize {
        let stream: GenericIterBox = self.chain.iter().fold(
            Box::new(Empty::default()), 
            |stream, 
                (cmd, args)| 
                    cmd(args, stream));

        stream.count()  // consume the iterator
    }
}

impl Default for Chain<'_> {
    fn default() -> Self {
        Chain { chain: Vec::new() }
    }
}
