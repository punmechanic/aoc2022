use aoc2022::Part;
use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::{self, stdin, Read},
};

// Add all of the problem modules here.
mod day1;

#[derive(Debug, Parser)]
#[command(name = "aoc2022", about = "Advent of Code 2022", long_about = None)]
struct Cli {
    #[command(subcommand)]
    problem: Commands,

    #[arg(short, long)]
    part: Part,

    #[arg(
        global = true,
        help = "The file to use as an input source. If this is the literal -, stdin is used instead.",
        default_value = "-"
    )]
    file: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Day1,
}

fn get_reader(name: &str) -> io::Result<Box<dyn Read>> {
    let read: Box<dyn Read> = match name {
        "-" => Box::new(stdin()),
        name => Box::new(File::open(name)?),
    };

    Ok(read)
}

fn main() {
    let args = Cli::parse();
    let reader = get_reader("-").unwrap();

    let result = match args.problem {
        Commands::Day1 => day1::execute(&args.part, reader),
    };

    result.unwrap();
}
