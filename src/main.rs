use std::{env, fs::read_to_string};

use csv_stream_language::{namespace::BuiltinNamespace, parse::parse_str};

fn main() {
    let args: Vec<String> = env::args().collect();

    let b_args: Vec<&str> = args.iter().map(String::as_str).collect();

    match b_args[..] {
        [] => {
            unreachable!();
        }
        [exec] => {
            eprintln!("Usage: {exec} file")
        }
        [_, ref cmd_line_args @ ..] => {
            let file = cmd_line_args[0];
            let file_content =
                read_to_string(file).unwrap_or_else(|_| panic!("Could not read file: {}", file));

            let chain = parse_str(&file_content, cmd_line_args, None::<BuiltinNamespace>);

            chain.execute();
        }
    }
}
