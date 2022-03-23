use crate::commands::{Command, Argument, GenericIterBox};


pub struct Chain<'a> {
    chain: Vec<(&'a Command, Vec<Argument>)>
}

impl<'a> Chain<'a> {
    pub fn push(&mut self, cmd: &'a Command, args: Vec<Argument>) {
        self.chain.push((cmd, args));
    }
}

impl Chain<'_> {
    pub fn execute(&self) {
        let mut current_data_stream: Option<GenericIterBox> = None;

        for (cmd, args) in self.chain.iter() {
            match cmd {
                Command::Begin(f) => {
                    if current_data_stream.is_some() {
                        panic!("Data stream already exists");
                    }

                    current_data_stream = Some( f(args) );
                },
                Command::Middle(f) => {
                    current_data_stream = Some( f(args, current_data_stream.expect("No data stream to process")) );
                },
                Command::End(f) => {
                    f(args, current_data_stream.expect("No data stream to process"));
                    current_data_stream = None;
                }
            }
        }
    }
}

impl Default for Chain<'_> {
    fn default() -> Self {
        Chain { chain: Vec::new() }
    }
}