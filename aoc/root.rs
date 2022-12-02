use aoc2022::Part;
use clap::{Parser, ValueEnum};
use std::{
    fs::File,
    io::{self, stdin, Read},
};

// Add all of the problem modules here.
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
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
    let reader = get_reader(&args.file).unwrap();
    let result = match args.problem {
        Problem::Day1 => day1::execute(&args.part, reader),
        Problem::Day2 => day2::execute(&args.part, reader),
        Problem::Day3 => day3::execute(&args.part, reader),
        Problem::Day4 => day4::execute(&args.part, reader),
        Problem::Day5 => day5::execute(&args.part, reader),
        Problem::Day6 => day6::execute(&args.part, reader),
        Problem::Day7 => day7::execute(&args.part, reader),
        Problem::Day8 => day8::execute(&args.part, reader),
        Problem::Day9 => day9::execute(&args.part, reader),
        Problem::Day10 => day10::execute(&args.part, reader),
        Problem::Day11 => day11::execute(&args.part, reader),
        Problem::Day12 => day12::execute(&args.part, reader),
        Problem::Day13 => day13::execute(&args.part, reader),
        Problem::Day14 => day14::execute(&args.part, reader),
        Problem::Day15 => day15::execute(&args.part, reader),
        Problem::Day16 => day16::execute(&args.part, reader),
        Problem::Day17 => day17::execute(&args.part, reader),
        Problem::Day18 => day18::execute(&args.part, reader),
        Problem::Day19 => day19::execute(&args.part, reader),
        Problem::Day20 => day20::execute(&args.part, reader),
        Problem::Day21 => day21::execute(&args.part, reader),
        Problem::Day22 => day22::execute(&args.part, reader),
        Problem::Day23 => day23::execute(&args.part, reader),
        Problem::Day24 => day24::execute(&args.part, reader),
        Problem::Day25 => day25::execute(&args.part, reader),
    };

    result.unwrap();
}
