// https://adventofcode.com/2022/day/2
use aoc2022::{Part, Result};
use itertools::Itertools;
use std::io::Read;

pub(crate) fn execute<R: Read>(part: &Part, mut reader: R) -> Result<()> {
    let mut lines: String = "".into();
    reader.read_to_string(&mut lines).unwrap();

    match part {
        Part::Part1 => println!("{}", solve1(&lines).unwrap()),
        Part::Part2 => println!("{}", solve2(&lines).unwrap()),
    };

    Ok(())
}

fn solve(input: &str, unique: usize) -> Option<u32> {
    let start: Vec<char> = input.chars().take(unique).collect();
    let rest = input.chars().skip(unique);
    let initial = (unique, start, None);
    let n = rest.into_iter().fold(initial, |(n, previous, value), c| {
        let new = [&previous[1..unique], &[c][..]].concat();
        match (n, new, value) {
            (n, next, value @ Some(_)) => (n, next, value),
            (n, next, _) if next.iter().unique().count() == unique => (n + 1, next, Some(n + 1)),
            (n, next, _) => (n + 1, next, None),
        }
    });

    n.2.map(|m| m as u32)
}

fn solve1(input: &str) -> Option<u32> {
    solve(input, 4)
}

fn solve2(input: &str) -> Option<u32> {
    solve(input, 14)
}

#[cfg(test)]

mod tests {
    use super::{solve1, solve2};

    #[test]
    fn solve1_works() {
        assert_eq!(Some(5), solve1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), solve1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(10), solve1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(11), solve1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn solve2_works() {
        assert_eq!(Some(19), solve2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(23), solve2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(23), solve2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(29), solve2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(26), solve2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
