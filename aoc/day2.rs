// https://adventofcode.com/2022/day/2
use std::{io::Read, str::FromStr};

pub(crate) fn execute<R: Read>(_part: &aoc2022::Part, mut _reader: R) -> aoc2022::Result<()> {
    unimplemented!();
}

#[derive(Debug)]
enum StrategyGuideError {
    InvalidFormatError,
    UnknownActionError,
}

impl From<UnknownActionError> for StrategyGuideError {
    fn from(_: UnknownActionError) -> Self {
        Self::UnknownActionError
    }
}

struct StrategyGuide {
    actions: Vec<(Action, Action)>,
}

impl FromStr for StrategyGuide {
    type Err = StrategyGuideError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guide = StrategyGuide {
            actions: Vec::new(),
        };

        for line in s.lines() {
            let parts: Vec<&str> = line.split(" ").collect();
            guide.actions.push((parts[0].parse()?, parts[1].parse()?));
        }

        Ok(guide)
    }
}

impl StrategyGuide {
    fn calculate_score(&self) -> u32 {
        0
    }
}

#[derive(Debug)]
struct UnknownActionError;

#[derive(Eq, PartialEq, Debug)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Action {
    type Err = UnknownActionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Action::Rock),
            "B" | "Y" => Ok(Action::Paper),
            "C" | "Z" => Ok(Action::Scissors),
            _ => Err(Self::Err {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::StrategyGuide;

    use super::Action;

    #[test]
    fn can_parse_steps() {
        let raw = "A Y
B X
C Z";

        let guide: StrategyGuide = raw.parse().expect("Failed to parse");
        assert_eq!(guide.actions[0], (Action::Rock, Action::Paper));
        assert_eq!(guide.actions[1], (Action::Paper, Action::Rock));
        assert_eq!(guide.actions[2], (Action::Scissors, Action::Scissors));
    }

    #[test]
    fn works_with_demo_strategy_guide() {
        let raw = "A Y
B X
C Z";
        let guide: StrategyGuide = raw.parse().unwrap();
        assert_eq!(guide.calculate_score(), 15);
    }
}
