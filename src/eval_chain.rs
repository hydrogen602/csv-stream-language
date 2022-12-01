/// The evaluation chain combines all the commands in a list and can execute an iterator through them
use crate::{
    commands::{Argument, Command, GenericIterBox},
    global_params::GlobalParams,
};
use std::{cell::RefCell, iter::Empty, rc::Rc};

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
        let params = Rc::new(RefCell::new(GlobalParams::default()));

        let stream: GenericIterBox = self
            .chain
            .into_iter()
            .fold(Box::new(Empty::default()), |stream, (cmd, args)| {
                cmd(args, stream, params.clone())
            });

        stream.count() // consume the iterator
    }

    pub fn execute_collect_out(self: Self) -> (usize, String) {
        let params = Rc::new(RefCell::new(GlobalParams::default().use_buffer()));

        let stream: GenericIterBox = self
            .chain
            .into_iter()
            .fold(Box::new(Empty::default()), |stream, (cmd, args)| {
                cmd(args, stream, params.clone())
            });

        let s = stream.count();
        let out = params.borrow_mut().get_buffer().unwrap();
        (s, out) // consume the iterator
    }
}

impl Default for Chain {
    fn default() -> Self {
        Chain { chain: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use crate::{namespace::BuiltinNamespace, parse::parse_str};

    #[test]
    fn execute_collect_out() {
        let chain = parse_str(
            "range 20 >> print >> sum 1 >> print",
            &[][..],
            None::<BuiltinNamespace>,
        );
        let (_, result) = chain.execute_collect_out();
        assert_eq!(
            result,
            "[0]\n[1]\n[2]\n[3]\n[4]\n[5]\n[6]\n[7]\n[8]\n[9]\n[10]\n[11]\n[12]\n[13]\n[14]\n[15]\n[16]\n[17]\n[18]\n[19]\n[190]\n"
            
        );
    }
}
