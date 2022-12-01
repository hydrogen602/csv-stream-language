use std::{cell::RefCell, mem::take, rc::Rc};

use crate::{
    commands::{Argument, DataTypes, GenericIterBox},
    global_params::GlobalParams,
    util,
};

pub struct LazyEval<F> {
    func: Option<F>,
}

impl<F> LazyEval<F> {
    pub fn new(func: F) -> Self {
        Self { func: Some(func) }
    }
}

impl<F, R> Iterator for LazyEval<F>
where
    F: FnOnce() -> R,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        take(&mut self.func).map(|f| f())
    }
}

pub fn sum(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    _: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    let pre_index = match util::to_1_tuple(m_args) {
        (Argument::Int(pre_index),) => pre_index,
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    };

    assert!(pre_index > 0, "index has to be greater than 0");
    let index = (pre_index - 1) as usize;

    Box::new(LazyEval::new(move || {
        let elem = input.fold(DataTypes::Int(0), |d, mut row| {
            let e = take(&mut row[index]);

            d + e
        });

        vec![elem]
    }))
}
