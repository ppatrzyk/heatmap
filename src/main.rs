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

struct CSVData {
    rows: Vec<Vec<Value>>,
    headers: Vec<String>,
    max_lengths: Vec<usize>,
    max_values: Vec<f64>,
    min_values: Vec<f64>,
}

fn process_file(file: &String) ->  std::result::Result<CSVData, Box<dyn Error>> {
    let mut reader = Reader::from_path(file)?;
    let headers = reader.headers()?.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let cols = headers.len();
    let mut max_lengths = (0usize..cols).map(|_x| 0).collect::<Vec<_>>();
    let mut max_values = (0usize..cols).map(|_x| f64::NEG_INFINITY).collect::<Vec<_>>();
    let mut min_values = (0usize..cols).map(|_x| f64::INFINITY).collect::<Vec<_>>();
    let mut rows: Vec<Vec<Value>> = Vec::new();
    for result in reader.records() {
        let record = result?;
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
        assert_eq!(parsed_row.len(), cols, "Bad length of row");
        rows.push(parsed_row);
    }
    let data = CSVData {
        rows: rows,
        headers: headers,
        max_lengths: max_lengths,
        min_values: min_values,
        max_values: max_values
    };
    Ok(data)
}

fn color_scale(val: f64, min_val: f64, max_val: f64) -> Color {
    let scale = colorous::YELLOW_ORANGE_RED;
    let normalized_val = (val-min_val)/max_val;
    println!("orig {:?}, normalized {:?}", val, normalized_val);
    let color = scale.eval_continuous(normalized_val);
    Color::Rgb { r: color.r, g: color.g, b: color.b }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match process_file(&args.file) {
        Err(reason) => {
            println!("Error reading {}: {}", &args.file, reason);
            Ok(())
        }
        Ok(data) => {
            println!("file read ok");
            let color = color_scale(50.0, -100.0, 300.0);
            println!("{:?}", color);
            println!("{:?}", data.max_lengths);
            println!("{:?}", data.max_values);
            println!("{:?}", data.min_values);
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
