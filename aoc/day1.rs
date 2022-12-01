// https://adventofcode.com/2022/day/1

use std::num::ParseIntError;

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
}

impl Elf {
    fn total_calories(&self) -> u32 {
        self.meals.iter().sum()
    }
}

#[derive(Default)]
struct ReducerState {
    line: usize,
    elves: Vec<Elf>,
}

fn parse_elves(inventory: &str) -> Result<Vec<Elf>, ParseError> {
    let initial = ReducerState::default();
    inventory
        .split_terminator("\n")
        .try_fold(initial, |mut state, line| {
            state.line += 1;

            match (line.is_empty(), str::parse::<u32>(line)) {
                (true, _) => {
                    state.elves.push(Elf::default());
                    Ok(state)
                }
                (false, Ok(calories)) => {
                    // If we reach this point, this is our first non-empty line that we've seen.
                    // This means we're about to add an Elf, and one needs to be present for our
                    // vector access to work.
                    if state.elves.len() == 0 {
                        state.elves.push(Elf::default());
                    }

                    let elf = state.elves.last_mut().expect(&format!(
                        "tried to add calories to an elf but there were none in the vec (line: {})",
                        state.line
                    ));

                    elf.meals.push(calories);
                    Ok(state)
                }
                (false, Err(e)) => Err((state.line, e).into()),
            }
        })
        .map(|r| r.elves)
}

fn find_maximum_calories(doc: &str) -> Result<Option<u32>, ParseError> {
    let elves = parse_elves(doc)?;
    let totals = elves.iter().map(|elf| elf.total_calories());
    Ok(totals.max())
}

fn find_top_three_highest_calories(doc: &str) -> Result<Option<u32>, ParseError> {
    let elves = parse_elves(doc)?;
    let mut calories: Vec<u32> = elves.iter().map(|elf| elf.total_calories()).collect();
    calories.sort_unstable();
    calories.reverse();

    let total = match &calories[..=2] {
        &[a, b, c] => Some(a + b + c),
        _ => None,
    };

    Ok(total)
}

fn main() {
    let doc = include_str!("day1.txt");
    match (
        find_maximum_calories(doc),
        find_top_three_highest_calories(doc),
    ) {
        (Ok(Some(max)), Ok(Some(top3))) => {
            println!("max calories: {}", max);
            println!("top 3 calories: {}", top3);
        }

        (Ok(None), _) | (_, Ok(None)) => {
            eprintln!("one of the samples received a blank document, but not both?");
        }

        (Err(err), _) | (_, Err(err)) => {
            eprintln!("line {} error: {}", err.lineno, err.reason)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Elf;

    use super::{find_maximum_calories, find_top_three_highest_calories, parse_elves};
    const TEST_DOCUMENT: &str = "
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
            Elf {
                meals: vec![1000, 2000, 3000],
            },
            Elf { meals: vec![4000] },
            Elf {
                meals: vec![5000, 6000],
            },
            Elf {
                meals: vec![7000, 8000, 9000],
            },
            Elf { meals: vec![10000] },
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
