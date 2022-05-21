use std::{io::{stdout}, error::Error};
use clap::{Parser};
use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor},
    ExecutableCommand, Result,
};
use csv::Reader;

// TODO
// accept stdin
// https://stackoverflow.com/questions/55148856/how-do-i-use-stdin-if-no-positional-arguments-are-given-with-clap
// header / no header option

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    file: String,
}

enum Value {
    String(String),
    Number(f64),
}

struct CSVData {
    rows: Vec<Vec<Value>>,
    max_lengths: Vec<usize>,
    max_values: Vec<f64>,
    min_values: Vec<f64>,
}

fn process_file(file: &String) ->  std::result::Result<CSVData, Box<dyn Error>> {
    let mut reader = Reader::from_path(file)?;
    let headers = reader.headers()?.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let cols = headers.len();
    let mut max_lengths = headers.iter().map(|x| x.len()).collect::<Vec<_>>();
    let mut max_values = (0usize..cols).map(|_x| f64::NEG_INFINITY).collect::<Vec<_>>();
    let mut min_values = (0usize..cols).map(|_x| f64::INFINITY).collect::<Vec<_>>();
    let mut rows: Vec<Vec<Value>> = Vec::new();
    rows.push(headers.iter().map(|x| Value::String(x.to_string())).collect::<Vec<_>>());
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
        max_lengths: max_lengths,
        min_values: min_values,
        max_values: max_values
    };
    Ok(data)
}

fn color_scale(val: f64, min_val: f64, max_val: f64) -> Color {
    // TODO guard against min==max
    let scale = colorous::YELLOW_ORANGE_RED;
    let normalized_val = (val-min_val)/(max_val-min_val);
    let color = scale.eval_continuous(normalized_val);
    Color::Rgb { r: color.r, g: color.g, b: color.b }
}

fn fixed_width(val: String, len: usize) -> String {
    let str_len = val.len();
    let whitespace = (0..(len-str_len)).map(|_| " ").collect::<String>();
    let mut formatted_str = " ".to_string();
    formatted_str.push_str(&val);
    formatted_str.push_str(&whitespace);
    formatted_str.push_str(" ");
    formatted_str
}

fn format_cell(val: &Value, len: usize, min_val: f64, max_val: f64) -> (String, Color) {
    match val {
        Value::String(val) =>
            (fixed_width(val.to_string(), len), Color::Reset),
        Value::Number(val) => 
            (fixed_width(val.to_string(), len), color_scale(*val, min_val, max_val))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match process_file(&args.file) {
        Err(reason) => {
            println!("Error reading {}: {}", &args.file, reason);
            Ok(())
        }
        Ok(data) => {
            for row in data.rows.iter() {
                stdout().execute(Print("|"))?;
                for (i, val) in row.iter().enumerate() {
                    let (val_str, color) = format_cell(val, data.max_lengths[i], data.min_values[i], data.max_values[i]);
                    stdout()
                    .execute(SetBackgroundColor(color))?
                    .execute(Print(val_str))?
                    .execute(ResetColor)?
                    .execute(Print("|"))?;
                }
                stdout().execute(Print("\n"))?;
            }
            Ok(())
        }
    }
}
