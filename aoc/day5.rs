// https://adventofcode.com/2022/day/2
use aoc2022::{Part, Result};
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

pub(crate) fn execute<R: Read>(part: &Part, reader: R) -> Result<()> {
    match part {
        Part::Part1 => println!("{}", solve1(reader).unwrap()),
        Part::Part2 => println!("{}", solve2(reader).unwrap()),
    };

    Ok(())
}

enum Mode {
    Instructions,
    Assignments,
}

fn solve1<R: Read>(reader: R) -> Result<String> {
    // Parse crate storage first.
    // The first n lines before a line that begins with a whitespace then a number are the crate allocations
    let buf = BufReader::new(reader);
    let mut assignments = Vec::new();
    let mut instructions = Vec::new();
    let mut mode = Mode::Assignments;
    for line in buf.lines() {
        let line = line.unwrap();
        match mode {
            Mode::Assignments => {
                if assignments.len() > 0 && line.is_empty() {
                    mode = Mode::Instructions;
                }

                assignments.push(line);
            }
            Mode::Instructions => {
                instructions.push(line);
            }
        };
    }

    // Each bucket will contain a Vec<string>. Each column in an assignment corresponds to one bucket.
    let mut buckets = HashMap::new();
    for line in assignments {
        if line.is_empty() {
            continue;
        }

        // dirty hack: we know how many columns to expect because each column is 3 characters with one character gutter
        for col in (0..line.len()).step_by(4) {
            let index = (col + 4) / 4;
            let start = col;
            let end = col + 3;
            let part = line[start..end].trim();
            if part.is_empty() {
                continue;
            }

            // If this character is a number, break out - we're not on the assignment line anymore.
            if part.parse::<u32>().is_ok() {
                break;
            }

            let letter = part.chars().nth(1).unwrap();
            buckets
                .entry(index as u32)
                .or_insert(Vec::new())
                .push(letter);
        }
    }

    // We need to reverse the buckets because they are currently in the wrong order;
    // we are reading the list from the top-down.
    for (_, bucket) in &mut buckets {
        bucket.reverse()
    }

    // process each instruction
    for instr in instructions {
        // each instruction is in a predictable format;
        // move N from X to Y
        // The crates act like a stack.
        let parts: Vec<&str> = instr.split(" ").collect();
        let count: u32 = parts[1].parse().unwrap();
        let src: u32 = parts[3].parse().unwrap();
        let dst: u32 = parts[5].parse().unwrap();
        let mut popped = Vec::new();
        let source = buckets.entry(src).or_default();
        for _ in 1..=count {
            popped.push(source.pop().unwrap());
        }

        let dst = buckets.entry(dst).or_default();
        // We replace them back on in reverse order, as they are transferred one by one and popped is currently a LIFO stack
        for el in popped {
            dst.push(el);
        }
    }

    let mut top = Vec::new();
    let sorted_keys = buckets.keys().sorted();
    for key in sorted_keys {
        let bucket = buckets.get(key).unwrap();
        top.push(bucket.last().unwrap());
    }

    Ok(top.into_iter().collect::<String>())
}
fn solve2<R: Read>(reader: R) -> Result<String> {
    // Parse crate storage first.
    // The first n lines before a line that begins with a whitespace then a number are the crate allocations
    let buf = BufReader::new(reader);
    let mut assignments = Vec::new();
    let mut instructions = Vec::new();
    let mut mode = Mode::Assignments;
    for line in buf.lines() {
        let line = line.unwrap();
        match mode {
            Mode::Assignments => {
                if assignments.len() > 0 && line.is_empty() {
                    mode = Mode::Instructions;
                }

                assignments.push(line);
            }
            Mode::Instructions => {
                instructions.push(line);
            }
        };
    }

    // Each bucket will contain a Vec<string>. Each column in an assignment corresponds to one bucket.
    let mut buckets = HashMap::new();
    for line in assignments {
        if line.is_empty() {
            continue;
        }

        // dirty hack: we know how many columns to expect because each column is 3 characters with one character gutter
        for col in (0..line.len()).step_by(4) {
            let index = (col + 4) / 4;
            let start = col;
            let end = col + 3;
            let part = line[start..end].trim();
            if part.is_empty() {
                continue;
            }

            // If this character is a number, break out - we're not on the assignment line anymore.
            if part.parse::<u32>().is_ok() {
                break;
            }

            let letter = part.chars().nth(1).unwrap();
            buckets
                .entry(index as u32)
                .or_insert(Vec::new())
                .push(letter);
        }
    }

    // We need to reverse the buckets because they are currently in the wrong order;
    // we are reading the list from the top-down.
    for (_, bucket) in &mut buckets {
        bucket.reverse()
    }

    // process each instruction
    for instr in instructions {
        // each instruction is in a predictable format;
        // move N from X to Y
        // The crates act like a stack.
        let parts: Vec<&str> = instr.split(" ").collect();
        let count: u32 = parts[1].parse().unwrap();
        let src: u32 = parts[3].parse().unwrap();
        let dst: u32 = parts[5].parse().unwrap();
        let mut popped = Vec::new();
        let source = buckets.entry(src).or_default();
        for _ in 1..=count {
            popped.push(source.pop().unwrap());
        }

        let dst = buckets.entry(dst).or_default();
        // This is the key difference in part 2; we apply the stack in the reverse order.
        popped.reverse();
        for el in popped {
            dst.push(el);
        }
    }

    let mut top = Vec::new();
    let sorted_keys = buckets.keys().sorted();
    for key in sorted_keys {
        let bucket = buckets.get(key).unwrap();
        top.push(bucket.last().unwrap());
    }

    Ok(top.into_iter().collect::<String>())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{solve1, solve2};

    const TEST_DOCUMENT: &[u8] = b"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn solve1_works() {
        assert_eq!(
            String::from("CMZ"),
            solve1(Cursor::new(TEST_DOCUMENT)).unwrap()
        );
    }

    #[test]
    fn solve2_works() {
        assert_eq!(
            String::from("MCD"),
            solve2(Cursor::new(TEST_DOCUMENT)).unwrap()
        );
    }
}
