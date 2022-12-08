use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::{
    commands::{Argument, DataTypes, GenericIterBox, RowType},
    global_params::GlobalParams,
    rule::MatchPattern,
    util::{self, GeneralError},
};

pub fn drop(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    _: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    match util::get_args(m_args) {
        [Argument::Enum(op), Argument::Int(n)] if op == "head" => {
            Box::new(input.skip(n as usize)) //input
        }
        [Argument::Enum(op), Argument::Int(_)] if op == "tail" => {
            todo!("tail not yet supported");
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub struct ColumnShuffle<I: Iterator<Item = RowType>> {
    it: I,
    order: Vec<usize>,
}

impl<I: Iterator<Item = RowType>> ColumnShuffle<I> {
    pub fn new(it: I, order: Vec<usize>) -> Self {
        ColumnShuffle { it, order }
    }
}

impl<I: Iterator<Item = RowType>> Iterator for ColumnShuffle<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.it
            .next()
            .map(|mrow| mrow.map(|row| self.order.iter().map(|&i| row[i].clone()).collect()))
    }
}

pub fn columns(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    _: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    match util::to_1_tuple(m_args) {
        (Argument::Tuple(order_arg),) => {
            let order: Vec<_> = order_arg
                .into_iter()
                .map(|e| {
                    if let Argument::Int(n) = e {
                        if n <= 0 {
                            panic!("Index columns start at 1");
                        }
                        // n > 0, so n-1 >= 0

                        (n - 1) as usize
                    } else {
                        panic!("Invalid argument: {:?}", e);
                    }
                })
                .collect();

            Box::new(ColumnShuffle::new(input, order))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn parse(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    _: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    // NaiveDate::parse_from_str
    match util::to_1_tuple(m_args) {
        (Argument::Tuple(args),) => {
            let types: Vec<String> = args
                .iter()
                .map(|e| {
                    if let Argument::Enum(ident) = e {
                        ident.into()
                    } else {
                        panic!("Invalid arguments: {:?}", args);
                    }
                })
                .collect();
            // let types = vec![1];

            Box::new(input.map(move |mrow| {
                mrow.and_then(|row| {
                    //let x = types;
                    if row.len() != types.len() {
                        Err(GeneralError::new(
                            "Error: Parse arguments don't match columns".into(),
                        ))
                    } else {
                        row.into_iter()
                            .zip(types.iter())
                            .map(|(data, ty)| match ty.as_str() {
                                "int" => Ok(DataTypes::Int(data.into())),
                                "float" => Ok(DataTypes::Float(data.into())),
                                "string" => Ok(DataTypes::String(data.into())),
                                "date" => Ok(DataTypes::NaiveDate(data.into())),
                                _ => Err(GeneralError::new(format!("unknown type: {}", ty))),
                            })
                            .collect()
                    }
                })
            }))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn filter(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    _: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    let (pre_index, pattern) = match util::to_2_tuple(m_args) {
        (Argument::Int(pre_index), Argument::Pattern(pattern)) => (pre_index, pattern),
        (Argument::Int(pre_index), Argument::String(ref pattern)) => {
            // there is no syntactic difference between a pattern and a string
            (pre_index, MatchPattern::compile_regex(pattern).unwrap())
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    };

    assert!(pre_index > 0, "index has to be greater than 0");
    let index = (pre_index - 1) as usize;

    Box::new(input.filter(move |mrow| {
        mrow.as_ref()
            .map(|row| {
                let elem = &row[index];
                pattern.is_match(elem)
            })
            .unwrap_or(true)
    }))
}

pub fn classify(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    _: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    match util::to_2_tuple(m_args) {
        (Argument::Int(pre_index), Argument::Tuple(args)) => {
            assert!(pre_index > 0, "index has to be greater than 0");
            let index = (pre_index - 1) as usize;

            let rules: Vec<_> = args
                .into_iter()
                .map(|e| {
                    if let Argument::Rule(rule, val) = e {
                        (rule, val)
                    } else {
                        panic!("Invalid arguments: expected rule, but got {:?}", e);
                    }
                })
                .collect();

            Box::new(input.map(move |mrow| {
                mrow.map(|mut row| {
                    let val = &row[index];

                    let label = rules
                        .iter()
                        .find_map(|(r, label)| {
                            if r.is_match(val) {
                                Some(label.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap_or(DataTypes::String("".to_string()));

                    //println!("{:?} {:?}", label, rules);

                    row.push(label);
                    row
                })
            }))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}
