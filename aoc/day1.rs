// https://adventofcode.com/2022/day/1
use clap::{Parser, ValueEnum};
use std::{
    fs::File,
    io::{self, stdin, Read},
    num::ParseIntError,
};

#[derive(Debug)]
struct ParseError {
    lineno: usize,
    reason: String,
}

impl From<(usize, ParseIntError)> for ParseError {
    fn from((n, err): (usize, ParseIntError)) -> Self {
        Self {
            lineno: n,
            reason: err.to_string(),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Elf {
    meals: Vec<u32>,
    total_calories: u32,
}

impl Elf {
    fn new() -> Self {
        Elf::default()
    }

    fn with_meals(meals: Vec<u32>) -> Self {
        let mut elf = Self::new();
        elf.meals = meals;
        elf.refresh_total_calories();
        elf
    }

    fn add_meal(&mut self, calories: u32) {
        self.meals.push(calories);
        self.total_calories += calories;
    }

    fn refresh_total_calories(&mut self) {
        let mut total = 0;
        for meal in &self.meals {
            total += meal;
        }

        self.total_calories = total;
    }
}

fn parse_elves<R: Read>(mut reader: R) -> Result<Vec<Elf>, ParseError> {
    let mut inventory = String::new();
    // This is bad but I do not care.
    reader.read_to_string(&mut inventory).unwrap();

    let initial: (usize, Vec<Elf>) = (0, Vec::new());
    inventory
        .split_terminator("\n")
        .try_fold(initial, |(n, mut elves), line| {
            match (line.is_empty(), str::parse::<u32>(line)) {
                // Acts as a "flush", putting a new elf on the stack.
                (true, _) => {
                    elves.push(Elf::new());
                    Ok((n, elves))
                }
                (false, Ok(calories)) => {
                    let elf = {
                        if elves.len() == 0 {
                            // This is our first non-empty line.
                            elves.push(Elf::new());
                        };

                        // It's not possible for this to be None because we already check if its empty above.
                        elves.last_mut().unwrap()
                    };

                    elf.add_meal(calories);
                    Ok((n, elves))
                }
                (false, Err(e)) => Err((n, e).into()),
            }
        })
        .map(|(_, elves)| elves)
}

fn find_maximum_calories<R: Read>(reader: R) -> Result<Option<u32>, ParseError> {
    let elves = parse_elves(reader)?;
    let totals = elves.iter().map(|elf| elf.total_calories);
    Ok(totals.max())
}

fn find_top_three_highest_calories<R: Read>(mut reader: R) -> Result<Option<u32>, ParseError> {
    let mut elves = parse_elves(&mut reader)?;
    elves.sort_unstable_by_key(|elf| elf.total_calories);
    elves.reverse();

    let total = match &elves[..=2] {
        [a, b, c] => Some(a.total_calories + b.total_calories + c.total_calories),
        _ => None,
    };

    Ok(total)
}

#[derive(ValueEnum, Clone)]
enum Mode {
    Part1,
    Part2,
}

#[derive(Parser)]
struct Program {
    #[arg(long)]
    mode: Mode,

    #[arg(name = "FILE", default_value = "-")]
    input_file: String,
}

impl Program {
    fn get_reader(&self) -> io::Result<Box<dyn Read>> {
        let read: Box<dyn Read> = match self.input_file.as_str() {
            "" | "-" => Box::new(stdin()),
            name => Box::new(File::open(name)?),
        };

        Ok(read)
    }
}

fn run(program: &Program) -> Result<(), ParseError> {
    let mut reader = match program.get_reader() {
        Err(e) => {
            eprintln!("Failed to open file for reader: {}", e);
            std::process::exit(1);
        }
        Ok(reader) => reader,
    };

    match program.mode {
        Mode::Part1 => {
            let calories = match find_maximum_calories(&mut reader)? {
                Some(cal) => cal,
                None => 0,
            };

            println!("{calories}");
        }
        Mode::Part2 => {
            let calories = match find_top_three_highest_calories(&mut reader)? {
                Some(cal) => cal,
                None => 0,
            };

            println!("{calories}");
        }
    };

    Ok(())
}

fn main() {
    let prog = Program::parse();
    if let Err(e) = run(&prog) {
        eprintln!("line {} error: {}", e.lineno, e.reason)
    }
}

#[cfg(test)]
mod tests {
    use super::{find_maximum_calories, find_top_three_highest_calories, parse_elves, Elf};
    const TEST_DOCUMENT: &[u8] = b"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn it_populates_elves_correctly() {
        let elves = &parse_elves(TEST_DOCUMENT).unwrap()[..];
        let expected = &[
            Elf::with_meals(vec![1000, 2000, 3000]),
            Elf::with_meals(vec![4000]),
            Elf::with_meals(vec![5000, 6000]),
            Elf::with_meals(vec![7000, 8000, 9000]),
            Elf::with_meals(vec![10000]),
        ][..];

        for n in 0..expected.len() {
            println!("{:?}", expected[n]);
            println!("{:?}", elves[n]);
            assert!(expected[n] == elves[n], "failed to match {n}th record");
        }
    }

    #[test]
    fn it_calculates_max_calories_of_elves() {
        assert_eq!(Some(24000), find_maximum_calories(TEST_DOCUMENT).unwrap());
    }

    #[test]
    fn it_calculates_calories_of_top_3_elves() {
        assert_eq!(
            Some(45000),
            find_top_three_highest_calories(TEST_DOCUMENT).unwrap()
        );
    }
}
