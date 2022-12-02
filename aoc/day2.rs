// https://adventofcode.com/2022/day/2
use std::{io::Read, str::FromStr};

use aoc2022::Part;

pub(crate) fn execute<R: Read>(part: &aoc2022::Part, mut reader: R) -> aoc2022::Result<()> {
    match part {
        Part::Part1 => {
            let mut raw = String::new();
            reader.read_to_string(&mut raw)?;
            let guide: StrategyGuide1 = raw.parse().unwrap();
            println!("{}", guide.calculate_score());
        }
        _ => unimplemented!(),
    }
    Ok(())
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

/// Determines if  A won or lost the round based on the given actions.
fn determine_round_outcome(a: Action, b: Action) -> Outcome {
    match (a, b) {
        (a, b) if a == b => Outcome::Draw,
        (Action::Paper, Action::Rock)
        | (Action::Scissors, Action::Paper)
        | (Action::Rock, Action::Scissors) => Outcome::Win,
        _ => Outcome::Loss,
    }
}

struct StrategyGuide1 {
    actions: Vec<(Action, Action)>,
}

impl FromStr for StrategyGuide1 {
    type Err = StrategyGuideError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guide = StrategyGuide1 {
            actions: Vec::new(),
        };

        for line in s.lines() {
            let parts: Vec<&str> = line.split(" ").collect();
            guide.actions.push((parts[0].parse()?, parts[1].parse()?));
        }

        Ok(guide)
    }
}

impl StrategyGuide1 {
    fn calculate_round_score(me: Action, opponent: Action) -> u32 {
        me.u32() + determine_round_outcome(me, opponent).u32()
    }

    /// Calculates the score of each action in the strategy guide.
    ///
    /// The score is equal to a fixed amount for the shape you played (1 for Rock, 2 for Paper and 3 for Scissors), plus an additional amount based on the outcome of the round; 0 for a loss, 3 for a draw and 6 for a win.
    fn calculate_score(&self) -> u32 {
        self.actions.iter().fold(0, |score, (opponent, me)| {
            score + Self::calculate_round_score(*me, *opponent)
        })
    }
}

#[derive(Debug)]
struct UnknownActionError;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Action {
    fn u32(&self) -> u32 {
        *self as u32
    }
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

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Outcome {
    fn u32(&self) -> u32 {
        *self as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::{determine_round_outcome, Action, Outcome, StrategyGuide1};

    #[test]
    fn has_correct_scores() {
        assert_eq!(Action::Rock.u32(), 1);
        assert_eq!(Action::Paper.u32(), 2);
        assert_eq!(Action::Scissors.u32(), 3);
        assert_eq!(Outcome::Win.u32(), 6);
        assert_eq!(Outcome::Draw.u32(), 3);
        assert_eq!(Outcome::Loss.u32(), 0);

        assert_eq!(
            determine_round_outcome(Action::Paper, Action::Rock),
            Outcome::Win
        );

        assert_eq!(
            determine_round_outcome(Action::Rock, Action::Paper),
            Outcome::Loss
        );

        assert_eq!(
            determine_round_outcome(Action::Scissors, Action::Paper),
            Outcome::Win
        );

        assert_eq!(
            determine_round_outcome(Action::Paper, Action::Scissors),
            Outcome::Loss
        );
    }

    #[test]
    fn can_parse_steps() {
        let raw = "A Y
B X
C Z";

        let guide: StrategyGuide1 = raw.parse().expect("Failed to parse");
        assert_eq!(guide.actions[0], (Action::Rock, Action::Paper));
        assert_eq!(guide.actions[1], (Action::Paper, Action::Rock));
        assert_eq!(guide.actions[2], (Action::Scissors, Action::Scissors));
    }

    #[test]
    fn works_with_demo_strategy_guide() {
        let raw = "A Y
B X
C Z";
        let guide: StrategyGuide1 = raw.parse().unwrap();
        assert_eq!(guide.calculate_score(), 15);
    }
}
