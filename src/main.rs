use std::{io::{stdout, Write}, error::Error};
use clap::{ArgEnum, Parser};
use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};
use csv::Reader;

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

fn process_file(file: &String) ->  std::result::Result<(i32, i32), Box<dyn Error>> {
    let mut reader = Reader::from_path(file)?;
    let headers = reader.headers()?;
    let cols = headers.len();
    println!("{:?}", headers);
    let mut max_lenghts = (0usize..cols).map(|_x| -1).collect::<Vec<_>>();
    let mut max_values = max_lenghts.clone();
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
        for value in record.iter() {
            match value.parse::<i32>() {
                Ok(n) => println!("Parsed ok {:?} -> {:?}", value, n), // TODO check maxes here and update, 
                Err(e) => println!("cannot parse {:?}", value), // TODO maybe try as float?
            }
        }
    }
    Ok((666, 5))
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
            .execute(SetForegroundColor(Color::Black))?
            .execute(SetBackgroundColor(Color::Red))?
            .execute(Print("test\n"))?
            .execute(SetBackgroundColor(Color::Green))?
            .execute(Print("test2"))?
            .execute(ResetColor)?;
            Ok(())
        }
    }
}
