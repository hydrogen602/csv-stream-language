use crate::{
    commands::{Argument, DataTypes, GenericIterBox, RowType},
    rule::MatchPattern,
    util,
};

pub fn drop(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
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
            .map(|row| self.order.iter().map(|&i| row[i].clone()).collect())
    }
}

pub fn columns(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    match util::to_1_tuple(m_args) {
        (Argument::Tuple(order_arg),) => {
            let order: Vec<_> = order_arg
                .into_iter()
                .map(|e| {
                    if let Argument::Int(n) = e {
                        if n <= 0 {
                            panic!("Index columns start at 1");
                        }
                        let index = (n - 1) as usize; // n > 0, so n-1 >= 0

                        index
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

pub fn parse(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
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

            Box::new(input.map(move |row| {
                //let x = types;
                if row.len() != types.len() {
                    panic!("Error: Parse arguments don't match columns")
                }

                row.into_iter()
                    .zip(types.iter())
                    .map(|(data, ty)| match ty.as_str() {
                        "int" => DataTypes::Int(data.into()),
                        "float" => DataTypes::Float(data.into()),
                        "string" => DataTypes::String(data.into()),
                        "date" => DataTypes::NaiveDate(data.into()),
                        _ => panic!("unknown type: {}", ty),
                    })
                    .collect()
            }))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn filter(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
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

    Box::new(input.filter(move |row| {
        let elem = &row[index];
        pattern.is_match(elem)
    }))
}

pub fn classify(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
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

            Box::new(input.map(move |mut row| {
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
            }))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}
