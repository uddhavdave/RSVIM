use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File path 
    #[clap(short, long, value_parser)]
    file_name: Option<String>,
}

/// Editor takes the path of the file as argument
pub fn parse() -> Option<String>{

    let args = Args::parse();

    return args.file_name;
}