use clap::Parser;
use std::path::PathBuf;
use mini_xml_parser::{element, Parser as XmlParser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match std::fs::read_to_string(cli.file) {
        Ok(s) => {
            match element().parse(&s) {
                Ok((_s, el)) => println!("{:?}", el),
                Err(err) => eprintln!("XML parsing error: {}", err),
            }
        }
        Err(err) => eprintln!("{:?}", err)
    }
}