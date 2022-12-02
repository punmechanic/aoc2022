use aoc2022::Part;
use clap::{Parser, ValueEnum};
use std::{
    fs::File,
    io::{self, stdin, Read},
};

// Add all of the problem modules here.
mod day1;
mod day2;

#[derive(Debug, Parser)]
#[command(name = "aoc2022", about = "Advent of Code 2022", long_about = None)]
struct Cli {
    #[arg(short, long, help = "The problem to run the given input against.")]
    problem: Problem,

    #[arg(long)]
    part: Part,

    #[arg(
        help = "The file to use as an input source. If this is the literal -, stdin is used instead.",
        default_value = "-"
    )]
    file: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum Problem {
    Day1,
    Day2,
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
        Problem::Day1 => day1::execute(&args.part, reader),
        Problem::Day2 => day2::execute(&args.part, reader),
    };

    result.unwrap();
}
