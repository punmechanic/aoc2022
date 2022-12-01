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
                // Acts as a "flush", putting a new elf on the stack.
                (true, _) => {
                    state.elves.push(Elf::new());
                    Ok(state)
                }
                (false, Ok(calories)) => {
                    let elf = {
                        if state.elves.len() == 0 {
                            // This is our first non-empty line.
                            state.elves.push(Elf::new());
                        };

                        // It's not possible for this to be None because we already check if its empty above.
                        state.elves.last_mut().unwrap()
                    };

                    elf.add_meal(calories);
                    Ok(state)
                }
                (false, Err(e)) => Err((state.line, e).into()),
            }
        })
        .map(|r| r.elves)
}

fn find_maximum_calories(doc: &str) -> Result<Option<u32>, ParseError> {
    let elves = parse_elves(doc)?;
    let totals = elves.iter().map(|elf| elf.total_calories);
    Ok(totals.max())
}

fn find_top_three_highest_calories(doc: &str) -> Result<Option<u32>, ParseError> {
    let elves = parse_elves(doc)?;
    let mut calories: Vec<u32> = elves.iter().map(|elf| elf.total_calories).collect();
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

    use super::{find_maximum_calories, find_top_three_highest_calories, parse_elves, Elf};
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
