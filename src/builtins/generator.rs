use std::{cell::RefCell, rc::Rc};

use crate::{
    commands::{Argument, DataTypes, GenericIterBox},
    global_params::GlobalParams,
    util,
};

pub fn range(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    _: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    fn helper(e: i32) -> Vec<DataTypes> {
        vec![DataTypes::Int(e)]
    }

    use either::Either::*;
    let it = match util::get_args_2_sizes(m_args) {
        Left([Argument::Int(stop)]) => (0..stop).map(helper),
        Right([Argument::Int(start), Argument::Int(stop)]) => (start..stop).map(helper),
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    };
    Box::new(input.chain(it))
}
