use csv::StringRecord;
use either::Either;

use crate::{commands::{Argument, GenericIterBox, RowType, DataTypes}, util, rule::MatchPattern};


/// Useful for repeats, just replacing anything with something
// macro_rules! replace {
//     ($r:expr, $_:tt) => {
//         $r
//     };
// }

// macro_rules! _reverse_helper {
//     ( [] $($p3:pat),+ ) => { ( $($p3),+ ) };
//     ( [ $p:pat, $($p2:pat),* ], $($p3:pat),* ) => {
//         _reverse_helper!( [ $($p2),* ] $p, $($p3),* )
//     };
// }

// macro_rules! reverse {
//     ( $($p2:pat),* ) => {
//         _reverse_helper!( [ $( $p2 ),* ], )
//     };
// }

/// Helps pattern matching arguments and printing error code
/// First is the container to match, then the arguments, comma separated, then the code
// macro_rules! arg_parser {
//     ($container:ident, $($p:pat),+ , $code:block) => {
//         {
//             let mut c = $container;
//             if (c.len() != 0usize $( +replace!(1usize, $p))+ ) { panic!("Wrong number of arguments: {:?}", c); }
//             match ( $( replace!(c.pop().unwrap(), $p)),+ , ) {
//                 (reverse!( $( $p ),+ )) => $code,
//                 no_match => {
//                     panic!("Wrong arguments: {:?}", no_match);
//                 }
//             }
//         }
//     };
//     // (2, $container:ident) => {
//     //     {
//     //         let mut c = $container;
//     //         if (c.len() != 2) { panic!("Wrong number of arguments: {:?}", c); }
//     //         (c.pop().unwrap(), c.pop().unwrap())
//     //     }
//     // };
//     // (3, $container:ident) => {
//     //     {
//     //         let mut c = $container;
//     //         if (c.len() != 3) { panic!("Wrong number of arguments: {:?}", c); }
//     //         (c.pop().unwrap(), c.pop().unwrap(), 
//     //             c.pop().unwrap())
//     //     }
//     // };
// }


pub fn read(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    fn read_file(file: &str) -> impl Iterator<Item=StringRecord> {
        let reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(file).expect("Could not open file for reading");

        let x = reader.into_records().map(|x| x.expect("Read failed"));
        x
    }

    match util::get_args_2_sizes(m_args) {
        Either::Left([Argument::String(file)]) => {
            // auto parse
            let it = read_file(&file).map(|sr: StringRecord| { sr.into_iter().map(|s| s.parse().unwrap()).collect() });

            Box::new(input.chain(it))
        },
        Either::Right([Argument::String(file), tup@Argument::Tuple(_)]) => {
            // custom parse
            let it = read_file(&file).map(|sr: StringRecord| { sr.into_iter().map(|s| DataTypes::String(s.to_string())).collect() });

            Box::new(parse(vec![tup], Box::new(input.chain(it))))
        }
        args => { panic!("Wrong arguments: {:?}", args); }
    }
}

pub fn write(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    match util::to_1_tuple(m_args) {
        (Argument::String(file),) => {
            let mut writer = csv::WriterBuilder::new().flexible(true).from_path(file).expect("Could not open file for writing");
    
            Box::new(input.map(move |row| {
                let x = row.iter().map(|x| x.to_string());
                writer.write_record(x).expect("Write failed");
    
                row
            }))
        },
        args => { 
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn drop(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    match util::get_args(m_args) {
        [Argument::Enum(op), Argument::Int(n)] if op == "head" => {
            Box::new(input.skip(n as usize)) //input
        },
        [Argument::Enum(op), Argument::Int(_)] if op == "tail" => {
            todo!("tail not yet supported");
        },
        args => { panic!("Wrong arguments: {:?}", args); }
    }
}

pub struct ColumnShuffle<I: Iterator<Item=RowType>> {
    it: I,
    order: Vec<usize>
}

impl<I: Iterator<Item=RowType>> ColumnShuffle<I> {
    pub fn new(it: I, order: Vec<usize>) -> Self {
        ColumnShuffle { it, order }
    }
}

impl<I: Iterator<Item=RowType>> Iterator for ColumnShuffle<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|row| {
            self.order.iter().map(|&i| row[i].clone()).collect()
        })
    }
}

pub fn columns(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    match util::to_1_tuple(m_args) {
        (Argument::Tuple(order_arg),) => {
            let order: Vec<_> = order_arg.into_iter().map(|e| {
                if let Argument::Int(n) = e {
                    if n <= 0 { panic!("Index columns start at 1"); }
                    let index = (n - 1) as usize; // n > 0, so n-1 >= 0

                    index
                }
                else {
                    panic!("Invalid argument: {:?}", e);
                }
            }).collect();

            Box::new( ColumnShuffle::new(input, order) )
        },
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn print(args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    if args.len() > 0 {
        panic!( "Invalid arguments: {:?}" , args );
    }

    Box::new(input.map(|row| {
        print!("[");
        for (i, elem) in row.iter().enumerate() {
            if i == 0 {
                print!("{}", elem);
            }
            else {
                print!(", {}", elem);
            }
        }
        println!("]");

        row
    }))
}

pub fn parse(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    // NaiveDate::parse_from_str
    match util::to_1_tuple(m_args) {
        (Argument::Tuple(args),) => {
            let types: Vec<String> = args.iter().map(|e| {
                if let Argument::Enum(ident) = e {
                    ident.into()
                }
                else {
                    panic!("Invalid arguments: {:?}", args);
                }
            }).collect();
            // let types = vec![1];

            Box::new(input.map(move |row| {
                //let x = types;
                if row.len() != types.len() {
                    panic!("Error: Parse arguments don't match columns")
                }

                row.into_iter().zip(types.iter()).map(|(data, ty)| {
                    match ty.as_str() {
                        "int" => DataTypes::Int(data.into()),
                        "float" => DataTypes::Float(data.into()),
                        "string" => DataTypes::String(data.into()),
                        "date" => DataTypes::NaiveDate(data.into()),
                        _ => panic!("unknown type: {}", ty)
                    }
                }).collect()
            }))
        },
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn filter(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
   let (pre_index, pattern) = match util::to_2_tuple(m_args) {
        (Argument::Int(pre_index), Argument::Pattern(pattern)) => {
            (pre_index, pattern)
        },
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

            let rules: Vec<_> = args.into_iter().map(|e| {
                if let Argument::Rule(rule, val) = e {
                    (rule, val)
                }
                else {
                    panic!("Invalid arguments: expected rule, but got {:?}", e);
                }
            }).collect();


            Box::new(input.map(move |mut row| {
                let val = &row[index];
                
                let label = rules.iter().find_map(
                    |(r, label)| if r.is_match(val) { Some(label.clone()) } else { None } 
                ).unwrap_or(DataTypes::String("".to_string()));

                //println!("{:?} {:?}", label, rules);
                
                row.push(label);
                row
            }))
        },
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}
pub mod summary {
    use std::mem::take;

    use super::*;

    pub struct LazyEval<F> {
        func: Option<F>
    }
    
    impl<F> LazyEval<F> {
        pub fn new(func: F) -> Self {
            Self { func: Some(func) }
        }
    }

    impl<F, R> Iterator for LazyEval<F> where F: FnOnce() -> R {
        type Item = R;
    
        fn next(&mut self) -> Option<Self::Item> {
            take(&mut self.func).map(|f| f())
        }
    }

    pub fn sum(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
        let pre_index = match util::to_1_tuple(m_args) {
            (Argument::Int(pre_index),) => {
                pre_index
            },
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
}

pub mod higher_order {

    // pub fn map(args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {

}


