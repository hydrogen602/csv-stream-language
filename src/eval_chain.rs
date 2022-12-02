/// The evaluation chain combines all the commands in a list and can execute an iterator through them
use crate::{
    commands::{Argument, Command, GenericIterBox},
    global_params::GlobalParams,
};
use std::{cell::RefCell, iter::Empty, rc::Rc, collections::HashMap};

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

    pub fn execute_collect_out(self: Self) -> (usize, String, HashMap<String, Vec<u8>>) {
        let params = Rc::new(RefCell::new(GlobalParams::default().use_buffer().capture_write_files()));

        let stream: GenericIterBox = self
            .chain
            .into_iter()
            .fold(Box::new(Empty::default()), |stream, (cmd, args)| {
                cmd(args, stream, params.clone())
            });

        let s = stream.count();
        let out = params.borrow_mut().get_buffer().unwrap();
        let written_data = params.borrow_mut().get_out_files().unwrap();
        (s, out, written_data) // consume the iterator
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
            "range 20 >> print >> write \"numbers.csv\" >> sum 1 >> print >> write \"test.csv\"",
            &[][..],
            None::<BuiltinNamespace>,
        );
        let (_, result, written_files) = chain.execute_collect_out();
        assert_eq!(
            result,
            "[0]\n[1]\n[2]\n[3]\n[4]\n[5]\n[6]\n[7]\n[8]\n[9]\n[10]\n[11]\n[12]\n[13]\n[14]\n[15]\n[16]\n[17]\n[18]\n[19]\n[190]\n"
            
        );
        assert_eq!(written_files["test.csv"], "190\n".as_bytes());
        assert_eq!(written_files["numbers.csv"], "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n".as_bytes())
    }
}
