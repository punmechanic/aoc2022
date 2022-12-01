// https://adventofcode.com/2022/day/1

#[derive(Debug)]
enum ParseError {
    ParseIntError {
        err: std::num::ParseIntError,
        lineno: usize,
    },
}

#[derive(Default, Debug)]
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
    current_elf: Elf,
}

fn parse_elves(inventory: &str) -> Result<Vec<Elf>, ParseError> {
    let initial = ReducerState::default();

    inventory
        .split_terminator("\n")
        .try_fold(initial, |mut state, line| {
            state.line += 1;
            if line.is_empty() {
                state.elves.push(state.current_elf);
                state.current_elf = Elf::default();
                return Ok(state);
            }

            match str::parse(line) {
                Ok(calories) => {
                    state.current_elf.meals.push(calories);
                }
                Err(e) => {
                    let err = ParseError::ParseIntError {
                        err: e,
                        lineno: state.line,
                    };

                    return Err(err);
                }
            }

            Ok(state)
        })
        .map(|r| r.elves)
}

fn find_maximum_calories(doc: &str) -> Result<Option<u32>, ParseError> {
    parse_elves(doc).map(|e| e.iter().map(|elf| elf.total_calories()).max())
}

fn main() {
    let doc = include_str!("day1.txt");
    match find_maximum_calories(doc) {
        Err(ParseError::ParseIntError { err, lineno }) => eprintln!("line {lineno} error: {err}"),
        Ok(None) => {
            eprintln!("list was empty?");
        }
        Ok(Some(calories)) => {
            println!("max calories: {}", calories)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_maximum_calories;

    #[test]
    fn works_on_sample_input() {
        let doc = "
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

        assert_eq!(
            Some(24000),
            find_maximum_calories(doc).expect("should have had no errors")
        );
    }
}
