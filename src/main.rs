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

fn process_file(file: &String) ->  std::result::Result<(LazyFrame, i32), PolarsError> {
    let df = LazyCsvReader::new(String::from(file))
    .has_header(true)
    .finish()?;

    let sample_df = df.clone().fetch(10)?;
    let dtypes = sample_df.dtypes();
    let colnames = sample_df.get_column_names();

    println!("{:?}", dtypes);
    println!("{:?}", colnames);

    Ok((df, 5))
}

fn main() -> Result<()> {
    let args = Args::parse();

    match process_file(&args.file) {
        Err(reason) => {
            println!("Error reading {}: {}", &args.file, reason);
            Ok(())
        }
        Ok((df, number)) => {
            println!("file read ok");
            stdout()
            .execute(SetForegroundColor(Color::Blue))?
            .execute(SetBackgroundColor(Color::Red))?
            .execute(Print("chuj"))?
            .execute(ResetColor)?;
            Ok(())
        }
    }
}
