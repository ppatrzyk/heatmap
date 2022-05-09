use std::io::{stdout, Write};
use clap::{ArgEnum, Parser};
use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};
use polars::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Row,
    Col,
    Full,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    file: String,

    #[clap(short, long, arg_enum, default_value = "col")]
    mode: Mode,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let df = LazyCsvReader::new(String::from(&args.file))
        .has_header(true)
        .finish();

    match df {
        Ok(_df) => println!("lazy frame read"),
        Err(_reason) => println!("error")
    }

    stdout()
    .execute(SetForegroundColor(Color::Blue))?
    .execute(SetBackgroundColor(Color::Red))?
    .execute(Print(&args.file))?
    .execute(ResetColor)?;
    
    Ok(())
}
