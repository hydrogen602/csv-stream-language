use csv::StringRecord;

use crate::commands::{Argument, GenericIterBox, RowType, DataTypes};

pub fn read(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    let file = if let [Argument::String(file)] = &args[..] { file } 
    else { 
        panic!("Invalid arguments: {:?}", args);
    };

    let reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(file).expect("Could not open file for reading");

    let x = reader.into_records().map(|x| x.expect("Read failed"));

    let it = x.map(|sr: StringRecord| { sr.into_iter().map(|s| s.into()).collect() });

    Box::new(input.chain(it))
}

pub fn write(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    let file = if let [Argument::String(file)] = &args[..] { file } 
    else { 
        panic!("Invalid arguments: {:?}", args);
    };

    let mut writer = csv::WriterBuilder::new().flexible(true).from_path(file).expect("Could not open file for writing");

    Box::new(input.map(move |row| {
        let x = row.iter().map(|x| x.to_string());
        writer.write_record(x).expect("Write failed");

        row
    }))
}

pub fn drop(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    if let [Argument::Enum(op), Argument::Int(n)] = &args[..] {
        match op.as_str() {
            "head" => {
                Box::new(input.skip(*n as usize)) //input
            },
            "tail" => {
                todo!("tail not yet supported");
            },
            arg => panic!("Invalid argument for drop: {}", arg)
        }
    } else {
        panic!("Invalid arguments: {:?}", args);
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
                panic!("Invalid arguments: {:?}", args);
            }
        }).collect();

        Box::new( ColumnShuffle::new(input, order) )
    }
    else {
        panic!("Invalid arguments: {:?}", args);
    }
}

pub fn print(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
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

pub fn parse(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    // NaiveDate::parse_from_str
    if let [Argument::Tuple(args)] = &args[..] {
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
    }
    else {
        panic!("Invalid arguments: {:?}", args);
    }
}


pub mod summary {
    // use super::*;
    // pub fn sum(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    //     //input.ma
    // }
}

pub mod higher_order {

    // pub fn map(args: &Vec<Argument>, input: GenericIterBox) -> GenericIterBox {

}

