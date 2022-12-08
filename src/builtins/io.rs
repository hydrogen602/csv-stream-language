use std::{cell::RefCell, rc::Rc};

use csv::StringRecord;
use either::Either;

use crate::{
    builtins::parse,
    commands::{Argument, DataTypes, GenericIterBox},
    global_params::GlobalParams,
    util::{self, GeneralError},
};

#[cfg(not(target_family = "wasm"))]
pub fn read(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    params: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    fn read_file(file: &str) -> impl Iterator<Item = Result<StringRecord, GeneralError>> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(file)
            .expect("Could not open file for reading");

        reader.into_records().map(|x| x.map_err(GeneralError::from))
    }

    match util::get_args_2_sizes(m_args) {
        Either::Left([Argument::String(file)]) => {
            // auto parse
            let it = read_file(&file).map(|m_sr| {
                m_sr.and_then(|sr| {
                    sr.into_iter()
                        .map(|s| Ok(s.parse::<DataTypes>().expect("This should never happen")))
                        .collect()
                })
            });

            Box::new(input.chain(it))
        }
        Either::Right([Argument::String(file), tup @ Argument::Tuple(_)]) => {
            // custom parse
            let it = read_file(&file).map(|m_sr| {
                m_sr.and_then(|sr| {
                    sr.into_iter()
                        .map(|s| Ok(DataTypes::String(s.to_string())))
                        .collect()
                })
            });

            Box::new(parse(vec![tup], Box::new(input.chain(it)), params))
        }
        args => Box::new(GeneralError::new(format!("Wrong arguments: {:?}", args)).iter()),
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
    use crate::commands::RowType;

    fn read_file<'a, R>(file: R) -> impl Iterator<Item = RowType> + 'a
    where
        R: std::io::Read + 'a,
    {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(file);

        reader.into_records().map(|m_sr| {
            m_sr.and_then(|sr| {
                sr.into_iter()
                    .map(|s| Ok(DataTypes::String(s.to_string())))
                    .collect()
            })
            .map_err(GeneralError::from)
        })
    }

    use std::mem;
    use util::OwnedStringRead;

    let mstdin = mem::take(&mut params.borrow_mut().input);
    match util::get_args_2_sizes(m_args) {
        Either::Left([]) => {
            // auto parse
            match mstdin {
                Some(stdin) => {
                    let reader = OwnedStringRead::new(stdin);

                    let it = read_file(reader);

                    Box::new(input.chain(it))
                }
                None => Box::new(input.chain(GeneralError::new("No input given".into()).iter())),
            }
        }
        Either::Right([tup @ Argument::Tuple(_)]) => {
            match mstdin {
                Some(stdin) => {
                    let reader = OwnedStringRead::new(stdin);

                    let it = read_file(reader);

                    Box::new(parse(vec![tup], Box::new(input.chain(it)), params))
                }
                None => Box::new(input.chain(GeneralError::new("No input given".into()).iter())),
            }
            // custom parse
        }
        args => Box::new(GeneralError::new(format!("Wrong arguments: {:?}", args)).iter()),
    }
}

pub fn write(
    m_args: Vec<Argument>,
    input: GenericIterBox,
    params: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    match util::to_1_tuple(m_args) {
        (Argument::String(file),) => {
            let writer = params
                .borrow_mut()
                .out_files
                .open_file(csv::WriterBuilder::new().flexible(true), &file);

            Box::new(input.map(move |m_row| {
                m_row.and_then(|row| {
                    let x = row.iter().map(|x| x.to_string());
                    (match &writer {
                        Either::Left(w) => w.borrow_mut().write_record(x),
                        Either::Right(w) => w.borrow_mut().write_record(x),
                    })?;

                    Ok(row)
                })
            }))
        }
        args => Box::new(GeneralError::new(format!("Wrong arguments: {:?}", args)).iter()),
    }
}

pub fn print(
    args: Vec<Argument>,
    input: GenericIterBox,
    params: Rc<RefCell<GlobalParams>>,
) -> GenericIterBox {
    if !args.is_empty() {
        Box::new(GeneralError::new(format!("Invalid arguments: {:?}", args)).iter())
    } else {
        Box::new(input.map(move |m_row| {
            let mut param_mut_ref = params.borrow_mut();
            use std::fmt::Write;
            write!(param_mut_ref.output, "[")?;
            m_row.and_then(|row| {
                for (i, elem) in row.iter().enumerate() {
                    if i == 0 {
                        write!(param_mut_ref.output, "{}", elem)?;
                    } else {
                        write!(param_mut_ref.output, ", {}", elem)?;
                    }
                }
                writeln!(param_mut_ref.output, "]")?;

                Ok(row)
            })
        }))
    }
}
