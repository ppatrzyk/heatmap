use std::io::{stdout, Write};
use clap::Parser;
use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};

// #[derive(Debug)]
// enum Mode {
//     Row,
//     Col,
//     Full,
// }

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file: String,

    #[clap(short, long, default_value = "row")]
    mode: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    stdout()
    .execute(SetForegroundColor(Color::Blue))?
    .execute(SetBackgroundColor(Color::Red))?
    .execute(Print(&args.file))?
    .execute(ResetColor)?;
    
    Ok(())
}
