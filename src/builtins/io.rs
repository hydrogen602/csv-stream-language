use std::{borrow::Borrow, cell::RefCell, mem, rc::Rc};

use csv::StringRecord;
use either::Either;

use crate::{
    builtins::parse,
    commands::{Argument, DataTypes, GenericIterBox},
    global_params::GlobalParams,
    util::{self, OwnedStringRead},
};

#[cfg(not(target_family = "wasm"))]
pub fn read(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    params: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    fn read_file(file: &str) -> impl Iterator<Item = StringRecord> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(file)
            .expect("Could not open file for reading");

        reader.into_records().map(|x| x.expect("Read failed"))
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

            Box::new(parse(vec![tup], Box::new(input.chain(it)), params))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

// #[cfg(target_family = "wasm")]

// struct ReadFromIn {
//     pub params: Rc<RefCell<GlobalParams>>,
//     pub reader: Option<csv::Reader<&[u8]>>,
// }

#[cfg(target_family = "wasm")]
pub fn read_in(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    params: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    fn read_file<'a, R>(file: R) -> impl Iterator<Item = StringRecord> + 'a
    where
        R: std::io::Read + 'a,
    {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(file);

        let x = reader.into_records().map(|x| x.expect("Read failed"));
        x
    }

    match util::get_args_2_sizes(m_args) {
        Either::Left([]) => {
            // auto parse
            let reader = OwnedStringRead::new(
                mem::take(&mut params.borrow_mut().input).expect("No input string"),
            );

            let it = read_file(reader).map(|sr: StringRecord| {
                sr.into_iter()
                    .map(|s| DataTypes::String(s.to_string()))
                    .collect()
            });

            Box::new(input.chain(it))
        }
        Either::Right([tup @ Argument::Tuple(_)]) => {
            let reader = OwnedStringRead::new(
                mem::take(&mut params.borrow_mut().input).expect("No input string"),
            );
            // custom parse
            let it = read_file(reader).map(|sr: StringRecord| {
                sr.into_iter()
                    .map(|s| DataTypes::String(s.to_string()))
                    .collect()
            });

            Box::new(parse(vec![tup], Box::new(input.chain(it)), params))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn write(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    params: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    match util::to_1_tuple(m_args) {
        (Argument::String(file),) => {
            // match &params.borrow().out_files {
            //     FILESYS => {}
            //     CAPTURE(m) => {}
            // }

            // let mut writer = match &params.borrow().out_files {
            //     inner_structs::OutFiles::CAPTURE(m) => {
            //         panic!()
            //     }
            //     inner_structs::OutFiles::FILESYS(m) => csv::WriterBuilder::new()
            //         .flexible(true)
            //         .from_path(file)
            //         .expect("Could not open file for writing"),
            // };

            let writer = params
                .borrow_mut()
                .out_files
                .open_file(csv::WriterBuilder::new().flexible(true), &file);

            Box::new(input.map(move |row| {
                let x = row.iter().map(|x| x.to_string());
                (match &writer {
                    Either::Left(w) => w.borrow_mut().write_record(x),
                    Either::Right(w) => w.borrow_mut().write_record(x),
                })
                .expect("Write failed");

                row
            }))
        }
        args => {
            panic!("Wrong arguments: {:?}", args);
        }
    }
}

pub fn print(
    args: Vec<Argument>,
    input: GenericIterBox,
    params: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    if !args.is_empty() {
        panic!("Invalid arguments: {:?}", args);
    }

    Box::new(input.map(move |row| {
        let mut param_mut_ref = params.borrow_mut();
        use std::fmt::Write;
        write!(param_mut_ref.output, "[").unwrap();
        for (i, elem) in row.iter().enumerate() {
            if i == 0 {
                write!(param_mut_ref.output, "{}", elem).unwrap();
            } else {
                write!(param_mut_ref.output, ", {}", elem).unwrap();
            }
        }
        writeln!(param_mut_ref.output, "]").unwrap();

        row
    }))
}
