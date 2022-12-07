use std::{cell::RefCell, rc::Rc};

use crate::{
    commands::{Argument, DataTypes, GenericIterBox, RowType},
    global_params::GlobalParams,
    util::{self, GeneralError},
};

pub fn range(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    _: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    fn helper(e: i32) -> RowType {
        Ok(vec![DataTypes::Int(e)])
    }

    use either::Either::*;
    match util::get_args_2_sizes(m_args) {
        Left([Argument::Int(stop)]) => Box::new(input.chain((0..stop).map(helper))),
        Right([Argument::Int(start), Argument::Int(stop)]) => {
            Box::new(input.chain((start..stop).map(helper)))
        }
        args => Box::new(
            input.chain(std::iter::once(RowType::Err(GeneralError::new(format!(
                "Wrong arguments: {:?}",
                args
            ))))),
        ),
    }
}
