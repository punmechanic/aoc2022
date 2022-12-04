/// --- Day 4: Camp Cleanup ---
///
/// Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.
///
/// However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).
///
/// For example, consider the following list of section assignment pairs:
///
///     2-4,6-8
///     2-3,4-5
///     5-7,7-9
///     2-8,3-7
///     6-6,4-6
///     2-6,4-8
///
/// For the first few pairs, this list means:
///
///     Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
///     The Elves in the second pair were each assigned two sections.
///     The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
///
/// This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:
///
///     .234.....  2-4
///     .....678.  6-8
///
///     .23......  2-3
///     ...45....  4-5
///
///     ....567..  5-7
///     ......789  7-9
///
///     .2345678.  2-8
///     ..34567..  3-7
///
///     .....6...  6-6
///     ...456...  4-6
///
///     .23456...  2-6
///     ...45678.  4-8
///
/// Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.
///
/// In how many assignment pairs does one range fully contain the other?
use aoc2022::{Part, Result};
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

pub(crate) fn execute<R: Read>(part: &Part, reader: R) -> Result<()> {
    match part {
        Part::Part1 => println!("{}", solve1(reader)),
        _ => todo!(),
    };

    Ok(())
}

const BITSET_LEN: u128 = u128::BITS as u128;

fn create_bitset(rangeish: &str) -> u128 {
    let parts: Vec<&str> = rangeish.split('-').collect();
    let start = parts[0].parse::<u128>().unwrap();
    let end = parts[1].parse::<u128>().unwrap();
    let mut x = 0u128;
    for n in 0..BITSET_LEN {
        if n <= end && n >= start {
            x |= 1 << n;
        }
    }

    x
}

fn solve1<R: Read>(reader: R) -> u128 {
    // We need to find the number of times assignment pairs _fully_ overlap.
    // Partial overlaps are OK - full overlaps are not.
    //
    // We only care about overlaps within a pair; the easiest way to do this would be to represent each assignment as a set of bit flags, then
    // step through each bit to see if both are set.
    let buf = BufReader::new(reader);
    let mut overlaps = 0;
    for res in buf.lines().filter_ok(|line| !line.is_empty()) {
        let line = res.unwrap();
        let parts = line.split(",").collect::<Vec<&str>>();
        let (a, b) = match parts[..] {
            [a, b] => (create_bitset(a), create_bitset(b)),
            _ => panic!("badly formed line"),
        };

        // Because there can be only one assignment per elf per pair, as long as one of the bitmasks fully envelops the other, we can say there's an overlap.
        // The easiest way to work this out is to BITAND the inverse of both and compare the flags.
        //
        // a & !b == 0 will determine if all of the bits in b were contained in a - if they were, then all of the bits in the result would be set to 0.
        if a & !b == 0 || b & !a == 0 {
            overlaps += 1
        }
    }

    overlaps
}

#[cfg(test)]
mod tests {
    use super::solve1;
    const SAMPLE_DOC: &[u8] = b"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn solve_part1_works() {
        assert_eq!(2, solve1(std::io::Cursor::new(SAMPLE_DOC)));
    }
}
