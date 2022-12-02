use std::collections::HashSet;

use aoc_helpers::prelude::*;

struct Day03;

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => ((c as u8) - b'a' + 1) as usize,
        'A'..='Z' => ((c as u8) - b'A' + 27) as usize,
        _ => panic!("Invalid character"),
    }
}

fn common_item_types(rucksuck: &Vec<char>) -> HashSet<char> {
    assert_eq!(rucksuck.len() % 2, 0);
    let first_compartment: HashSet<char> =
        rucksuck.iter().take(rucksuck.len() / 2).copied().collect();
    let second_compartment: HashSet<char> =
        rucksuck.iter().skip(rucksuck.len() / 2).copied().collect();
    first_compartment
        .intersection(&second_compartment)
        .copied()
        .collect()
}

fn common_item_types_in_group(rucksucks: &[Vec<char>]) -> HashSet<char> {
    let mut commons: HashSet<char> = rucksucks[0].iter().copied().collect();
    for rucksuck in &rucksucks[1..] {
        let items: HashSet<char> = rucksuck.iter().copied().collect();
        for item in commons.clone().into_iter() {
            if !items.contains(&item) {
                commons.remove(&item);
            }
        }
    }
    commons
}

impl Problem for Day03 {
    type Input = RowsOfChars<char>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input
            .iter()
            .map(|rucksuck| {
                common_item_types(rucksuck)
                    .into_iter()
                    .map(priority)
                    .sum::<usize>()
            })
            .sum()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        assert_eq!(input.len() % 3, 0);
        input
            .chunks(3)
            .map(|group| {
                common_item_types_in_group(group)
                    .into_iter()
                    .map(priority)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    solve::<Day03>(include_str!("../../inputs/day03.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day03>(SAMPLE), 157);
        assert_eq!(solve_part2::<Day03>(SAMPLE), 70);
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('B'), 28);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_common_item_types() {
        let rucksuck: Vec<char> = "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect();
        let commons = common_item_types(&rucksuck);
        assert_eq!(commons.into_iter().collect::<Vec<char>>(), vec!['p']);
    }
}
