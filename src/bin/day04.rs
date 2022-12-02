use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day04;

#[derive(Clone, Copy, Debug)]
#[rematch(r"([0-9]+)-([0-9]+)")]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains_fully(&self, other: &Range) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn contains(&self, point: usize) -> bool {
        self.start <= point && point <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
    }
}

#[derive(Clone, Copy, Debug)]
#[rematch(r"([0-9]+-[0-9]+),([0-9]+-[0-9]+)")]
struct RangePair {
    first: Range,
    second: Range,
}

impl RangePair {
    fn contains_fully(&self) -> bool {
        self.first.contains_fully(&self.second) || self.second.contains_fully(&self.first)
    }

    fn has_overlap(&self) -> bool {
        self.first.overlaps(&self.second)
    }
}

impl Problem for Day04 {
    type Input = VecFromLines<RangePair>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.iter().filter(|pair| pair.contains_fully()).count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input.iter().filter(|pair| pair.has_overlap()).count()
    }
}

fn main() {
    solve::<Day04>(include_str!("../../inputs/day04.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day04>(SAMPLE), 2);
        assert_eq!(solve_part2::<Day04>(SAMPLE), 4);
    }
}
