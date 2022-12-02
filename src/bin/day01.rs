use aoc_helpers::prelude::*;

struct Day01;

impl Problem for Day01 {
    type Input = VecFromMultiLines<VecFromLines<usize>>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input
            .iter()
            .map(|nums| nums.iter().copied().sum())
            .max()
            .expect("There should be at least one number in the input")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut totals: Vec<usize> = input
            .iter()
            .map(|nums| nums.iter().copied().sum())
            .collect();
        totals.sort_unstable();
        totals.reverse();
        assert!(totals.len() >= 3);
        totals[0] + totals[1] + totals[2]
    }
}

fn main() {
    solve::<Day01>(include_str!("../../inputs/day01.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day01>(SAMPLE), 24000);
        assert_eq!(solve_part2::<Day01>(SAMPLE), 45000);
    }
}
