use csv::StringRecord;
use either::Either;

use crate::{
    builtins::parse,
    commands::{Argument, DataTypes, GenericIterBox},
    util,
};

pub fn read(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    fn read_file(file: &str) -> impl Iterator<Item = StringRecord> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(file)
            .expect("Could not open file for reading");

        let x = reader.into_records().map(|x| x.expect("Read failed"));
        x
    }

    match util::get_args_2_sizes(m_args) {
        Either::Left([Argument::String(file)]) => {
            // auto parse
            let it = read_file(&file)
                .map(|sr: StringRecord| sr.into_iter().map(|s| s.parse().unwrap()).collect());

            Box::new(input.chain(it))
        }
        Either::Right([Argument::String(file), tup @ Argument::Tuple(_)]) => {
            // custom parse
            let it = read_file(&file).map(|sr: StringRecord| {
                sr.into_iter()
                    .map(|s| DataTypes::String(s.to_string()))
                    .collect()
            });

            Box::new(parse(vec![tup], Box::new(input.chain(it))))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn write(m_args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    match util::to_1_tuple(m_args) {
        (Argument::String(file),) => {
            let mut writer = csv::WriterBuilder::new()
                .flexible(true)
                .from_path(file)
                .expect("Could not open file for writing");

            Box::new(input.map(move |row| {
                let x = row.iter().map(|x| x.to_string());
                writer.write_record(x).expect("Write failed");

                row
            }))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn print(args: Vec<Argument>, input: GenericIterBox) -> GenericIterBox {
    if args.len() > 0 {
        panic!("Invalid arguments: {:?}", args);
    }

    Box::new(input.map(|row| {
        print!("[");
        for (i, elem) in row.iter().enumerate() {
            if i == 0 {
                print!("{}", elem);
            } else {
                print!(", {}", elem);
            }
        }
        println!("]");

        row
    }))
}
