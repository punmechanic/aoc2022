/// --- Day 3: Rucksack Reorganization ---
/// One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
///
/// Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
///
/// The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).
///
/// The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
///
/// For example, suppose you have the following list of contents from six rucksacks:
///
///     vJrwpWtwJgWrhcsFMMfFFhFp
///     jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
///     PmmdzqPrVvPwwTWBwg
///     wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
///     ttgJtRGJQctTZtZT
///     CrZsJsPPZsGzwwsLwLmpwMDw
///
///  * The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
///  * The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
///  * The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
///  * The fourth rucksack's compartments only share item type v.
///  * The fifth rucksack's compartments only share item type t.
///  * The sixth rucksack's compartments only share item type s.
///
/// To help prioritize item rearrangement, every item type can be converted to a priority:
///
///  * Lowercase item types a through z have priorities 1 through 26.
///  * Uppercase item types A through Z have priorities 27 through 52.
///
/// In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
///
/// Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
use aoc2022::{Part, Result};
use std::io::{BufRead, BufReader, Read};

pub(crate) fn execute<R: Read>(part: &Part, reader: R) -> Result<()> {
    match part {
        Part::Part1 => println!("{}", solve(reader)?),
        _ => unimplemented!(),
    };

    Ok(())
}

#[derive(Debug)]
struct ErrControlCharacter;

fn priority(ch: char) -> std::result::Result<u8, ErrControlCharacter> {
    match ch as u8 {
        c if c > 96 => Ok(c - 96),
        // Upper case letters need to have 27 added, because the minimum priority they can be is 27
        c if c < 91 => Ok(c - 65 + 27),
        _ => Err(ErrControlCharacter {}),
    }
}

fn solve<R: Read>(reader: R) -> Result<u32> {
    let buf = BufReader::new(reader);
    let res = buf.lines().fold(0u32, |acc, line| {
        let line = line.unwrap();
        if line.len() == 0 {
            return acc;
        }

        let mid = line.len() / 2;
        let halves = (&line[0..=mid], &line[mid..]);
        let mut shared = None;
        for c in halves.0.chars() {
            if halves.1.contains(c) {
                shared = Some(c);
                break;
            }
        }

        acc + priority(shared.unwrap()).unwrap() as u32
    });

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE_DATA: &[u8] = b"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn solve_gives_correct_answer_for_sample_data() {
        assert_eq!(Ok(157), solve(SAMPLE_DATA));
    }
}
