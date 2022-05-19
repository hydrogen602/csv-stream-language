/// The evaluation chain combines all the commands in a list and can execute an iterator through them
use crate::commands::{Argument, Command, GenericIterBox};
use std::iter::Empty;

pub struct Chain {
    chain: Vec<(Command, Vec<Argument>)>,
}

impl Chain {
    pub fn push(&mut self, cmd: Command, args: Vec<Argument>) {
        self.chain.push((cmd, args));
    }
}

impl Chain {
    pub fn execute(self: Self) -> usize {
        let stream: GenericIterBox = self
            .chain
            .into_iter()
            .fold(Box::new(Empty::default()), |stream, (cmd, args)| {
                cmd(args, stream)
            });

        stream.count() // consume the iterator
    }
}

impl Default for Chain {
    fn default() -> Self {
        Chain { chain: Vec::new() }
    }
}
