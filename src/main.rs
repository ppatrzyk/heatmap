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
    let mut max_lengths = (0usize..cols).map(|_x| 0).collect::<Vec<_>>();
    let mut max_values = (0usize..cols).map(|_x| f64::NEG_INFINITY).collect::<Vec<_>>();
    let mut min_values = (0usize..cols).map(|_x| f64::INFINITY).collect::<Vec<_>>();
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
        for (i, value) in record.iter().enumerate() {
            let entry_len = value.len();
            if entry_len > max_lengths[i] {
                max_lengths[i] = entry_len;
            }
            match value.parse::<f64>() {
                Ok(n) => {
                    if n > max_values[i] {
                        max_values[i] = n;
                    }
                    if n < min_values[i] {
                        min_values[i] = n;
                    }
                }
                Err(_e) => ()
            }
            // TODO append and track vals
        }
    }
    println!("{:?}", max_lengths);
    println!("{:?}", max_values);
    println!("{:?}", min_values);
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
