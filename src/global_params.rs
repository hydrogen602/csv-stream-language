use std::{collections::HashMap, io::stdout, mem, rc::Rc};

pub mod inner_structs {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Stdout;

    pub enum Output {
        STDOUT(Stdout),
        BUFFER(String),
    }

    use std::fmt;
    use std::rc::Rc;

    use either::Either::{self, Left, Right};

    impl fmt::Write for Output {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            use std::io::Write;
            match self {
                Self::BUFFER(buffer) => buffer.write_str(s),
                Self::STDOUT(handle) => match write!(handle, "{}", s) {
                    Ok(_) => fmt::Result::Ok(()),
                    Err(_) => fmt::Result::Err(fmt::Error),
                },
            }
        }
    }

    pub enum OutFiles {
        FILESYS(HashMap<String, Rc<RefCell<csv::Writer<File>>>>),
        CAPTURE(HashMap<String, Rc<RefCell<csv::Writer<Vec<u8>>>>>),
    }

    impl OutFiles {
        //std::fs::File
        pub fn open_file<'a>(
            &'a mut self,
            builder: &csv::WriterBuilder,
            file: &str,
        ) -> Either<Rc<RefCell<csv::Writer<File>>>, Rc<RefCell<csv::Writer<Vec<u8>>>>> {
            match self {
                Self::FILESYS(map) => {
                    let writer =
                        RefCell::new(builder.from_path(file).expect("Could not open file"));
                    map.insert(file.into(), Rc::new(writer));
                    Left(map[file].clone())
                }
                Self::CAPTURE(map) => {
                    let writer = RefCell::new(builder.from_writer(vec![]));
                    map.insert(file.into(), Rc::new(writer));
                    Right(map[file].clone())
                }
            }
        }
    }
}

pub struct GlobalParams {
    pub output: inner_structs::Output,
    pub out_files: inner_structs::OutFiles,
}

impl GlobalParams {
    pub fn new() -> Self {
        GlobalParams {
            output: inner_structs::Output::STDOUT(stdout()),
            out_files: inner_structs::OutFiles::FILESYS(HashMap::new()),
        }
    }

    pub fn use_buffer(mut self) -> Self {
        self.output = inner_structs::Output::BUFFER(String::new());
        self
    }

    pub fn capture_write_files(mut self) -> Self {
        self.out_files = inner_structs::OutFiles::CAPTURE(HashMap::new());
        self
    }

    pub fn get_buffer(&mut self) -> Option<String> {
        if let inner_structs::Output::BUFFER(ref mut s) = self.output {
            Some(mem::replace(s, String::new()))
        } else {
            None
        }
    }

    pub fn get_out_files(&mut self) -> Option<HashMap<String, Vec<u8>>> {
        match &mut self.out_files {
            inner_structs::OutFiles::FILESYS(_) => None,
            inner_structs::OutFiles::CAPTURE(m) => {
                let extracted = mem::replace(m, HashMap::new());
                let files: HashMap<String, Vec<u8>> = extracted
                    .into_iter()
                    .map(|(k, v)| {
                        let data = Rc::try_unwrap(v)
                            .expect("There shouldn't be multiple references alive, but there are")
                            .into_inner()
                            .into_inner()
                            .expect("csv writer failed");

                        (k, data)
                    })
                    .collect();

                Some(files)
            }
        }
    }
}

impl Default for GlobalParams {
    fn default() -> Self {
        Self::new()
    }
}
