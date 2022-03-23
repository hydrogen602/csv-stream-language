use csv::StringRecord;

use crate::commands::{Argument, GenericIterBox, RowType};

pub fn read(args: &Vec<Argument>) -> GenericIterBox {
    let file = if let [Argument::String(file)] = &args[..] { file } 
    else { 
        unreachable!();
    };

    let reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(file).expect("Could not read file");

    let x = reader.into_records().map(|x| x.expect("Read failed"));

    let it = x.map(|sr: StringRecord| { sr.into_iter().map(|s| s.into()).collect() });

    Box::new(it)
}

// pub enum DropIterator<I: Iterator> {
//     HeadDrop(I),
//     //TailDrop(I, queues::CircularBuffer<>)
// }

// impl<I: Iterator> Iterator for DropIterator<I> {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self {
//             Self::HeadDrop(it) => { it.next() }
//         }
//     }
// }

// impl<I: Iterator> DropIterator<I> {
//     fn head(it: GenericIterBox, skip_rows: usize) -> Self {
//         for _ in 0..skip_rows {
//             if let None = it.next() {
//                 return Self::HeadDrop(iter::empty());
//             }
//         }
//         Self::HeadDrop(*it)
//     }
// }

pub fn drop(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    if let [Argument::Enum(op), Argument::Int(n)] = &args[..] {
        match op.as_str() {
            "head" => {
                let x = input.skip(*n as usize);
                // for _ in 0..*n {
                //     if let None = input.next() {
                //         break;
                //     }
                // }
                Box::new(x) //input
            },
            "tail" => {
                todo!("tail not yet supported");
            },
            arg => panic!("Invalid argument for drop: {}", arg)
        }
    } else {
        unreachable!();
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

pub fn columns(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    if let [Argument::Tuple(order_arg)] = &args[..] {
        let order: Vec<_> = order_arg.into_iter().map(|e| {
            if let Argument::Int(n) = *e {
                if n <= 0 { panic!("Index columns start at 1"); }
                let index = (n - 1) as usize; // n > 0, so n-1 >= 0

                index
            }
            else {
                unreachable!();
            }
        }).collect();

        Box::new( ColumnShuffle::new(input, order) )
    }
    else {
        unreachable!();
    }
}

pub fn print(args: &Vec<Argument>, input: GenericIterBox) {
    if args.len() > 0 {
        panic!( "Invalid arguments: {:?}" , args );
    }

    for row in input {
        print!("[");
        for (i, elem) in row.into_iter().enumerate() {
            if i == 0 {
                print!("{}", elem);
            }
            else {
                print!(", {}", elem);
            }
        }
        println!("]");
    }
}

