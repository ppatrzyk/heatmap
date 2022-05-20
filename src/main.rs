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

enum Value {
    String(String),
    Number(f64),
}

fn process_file(file: &String) ->  std::result::Result<(Vec<Vec<Value>>, Vec<usize>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut reader = Reader::from_path(file)?;
    let headers = reader.headers()?;
    let cols = headers.len();
    println!("{:?}", headers);
    let mut max_lengths = (0usize..cols).map(|_x| 0).collect::<Vec<_>>();
    let mut max_values = (0usize..cols).map(|_x| f64::NEG_INFINITY).collect::<Vec<_>>();
    let mut min_values = (0usize..cols).map(|_x| f64::INFINITY).collect::<Vec<_>>();
    let mut rows: Vec<Vec<Value>> = Vec::new();
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
        let mut parsed_row: Vec<Value> = Vec::with_capacity(cols);
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
                    parsed_row.push(Value::Number(n))
                }
                Err(_e) => {
                    parsed_row.push(Value::String(value.to_string()))
                }
            }
        }
        rows.push(parsed_row);
    }
    Ok((rows, max_lengths, max_values, min_values))
}

fn main() -> Result<()> {
    let args = Args::parse();

    match process_file(&args.file) {
        Err(reason) => {
            println!("Error reading {}: {}", &args.file, reason);
            Ok(())
        }
        Ok((rows, max_lengths, max_values, min_values)) => {
            println!("file read ok");
            println!("{:?}", max_lengths);
            println!("{:?}", max_values);
            println!("{:?}", min_values);
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
