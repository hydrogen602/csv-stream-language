use std::{io::stdout, mem};

pub mod inner_structs {
    use std::io::{Stdout, Write};

    pub enum Output {
        STDOUT(Stdout),
        BUFFER(String),
    }

    // impl Write for Output {
    //     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    //         match self {
    //             Self::STDOUT(out) => out.write(buf),
    //             Self::BUFFER(out) => out.write(buf),
    //         }
    //     }

    //     fn flush(&mut self) -> std::io::Result<()> {
    //         match self {
    //             Self::STDOUT(out) => out.flush(),
    //             Self::BUFFER(_) => Ok(()),
    //         }
    //     }
    // }
    use std::fmt;
    impl fmt::Write for Output {
        // fn write_str(&mut self, s: &str) -> fmt::Result {
        //     match self.write(s.as_bytes()) {
        //         std::io::Result::Err(_) => fmt::Result::Err(fmt::Error),
        //         std::io::Result::Ok(_) => fmt::Result::Ok(()),
        //     }
        // }
        fn write_str(&mut self, s: &str) -> fmt::Result {
            match self {
                Self::BUFFER(buffer) => buffer.write_str(s),
                Self::STDOUT(handle) => match write!(handle, "{}", s) {
                    Ok(_) => fmt::Result::Ok(()),
                    Err(_) => fmt::Result::Err(fmt::Error),
                },
            }
        }
    }
}

pub struct GlobalParams {
    pub output: inner_structs::Output,
}

impl GlobalParams {
    pub fn new() -> Self {
        GlobalParams {
            output: inner_structs::Output::STDOUT(stdout()),
        }
    }

    pub fn use_buffer(mut self) -> Self {
        self.output = inner_structs::Output::BUFFER(String::new());
        self
    }

    pub fn get_buffer(&mut self) -> Option<String> {
        if let inner_structs::Output::BUFFER(ref mut s) = self.output {
            Some(mem::replace(s, String::new()))
        } else {
            None
        }
    }
}

impl Default for GlobalParams {
    fn default() -> Self {
        Self::new()
    }
}
