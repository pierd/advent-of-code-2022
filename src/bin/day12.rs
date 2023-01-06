use std::collections::HashMap;

use aoc_helpers::{prelude::*, walk::Walker};

const START: char = 'S';
const END: char = 'E';

struct Day12;

type Coords = (usize, usize);

struct PathFinder<'a> {
    steps: HashMap<Coords, usize>,
    map: &'a Vec<Vec<char>>,
    target: char,
    uphill: bool,
}

impl<'a> PathFinder<'a> {
    fn new(map: &'a Vec<Vec<char>>, target: char, uphill: bool) -> Self {
        Self {
            steps: Default::default(),
            map,
            target,
            uphill,
        }
    }
}

struct NextStepGenerator<'a> {
    map: &'a Vec<Vec<char>>,
    from: Coords,
    steps: usize,
    uphill: bool,
}

fn normalise(c: char) -> usize {
    if c == START {
        0
    } else if c == END {
        (b'z' - b'a') as usize
    } else {
        assert!(('a'..='z').contains(&c));
        (c as u8 - b'a') as usize
    }
}

impl<'a> walk::Generator<(usize, Coords)> for NextStepGenerator<'a> {
    fn generate<F: FnMut((usize, Coords))>(&mut self, mut callback: F) {
        let rows = self.map.len();
        let cols = self.map[0].len();
        let current = normalise(self.map[self.from.0][self.from.1]);
        let current_row = self.from.0 as isize;
        let current_col = self.from.1 as isize;
        let mut try_offset = |drow: isize, dcol: isize| {
            if let (Some(target_row), Some(target_col)) = (
                usize::try_from(current_row + drow).ok(),
                usize::try_from(current_col + dcol).ok(),
            ) {
                if target_row < rows
                    && target_col < cols
                    && ((self.uphill && normalise(self.map[target_row][target_col]) <= current + 1)
                        || (!self.uphill
                            && current <= normalise(self.map[target_row][target_col]) + 1))
                {
                    callback((self.steps, (target_row, target_col)));
                }
            }
        };
        try_offset(1, 0);
        try_offset(0, 1);
        try_offset(-1, 0);
        try_offset(0, -1);
    }
}

impl<'a> Walker<(usize, Coords)> for PathFinder<'a> {
    type NextGenerator = NextStepGenerator<'a>;

    type Result = usize;

    fn visit(
        &mut self,
        (steps, (row, col)): &(usize, Coords),
    ) -> walk::VisitDecision<Self::Result, Self::NextGenerator> {
        if self.map[*row][*col] == self.target {
            return walk::VisitDecision::Break(*steps);
        }
        if let Some(previous_steps) = self.steps.get(&(*row, *col)).copied() {
            if previous_steps <= *steps {
                return walk::VisitDecision::Continue;
            }
        }
        self.steps.insert((*row, *col), *steps);
        walk::VisitDecision::Next(NextStepGenerator {
            map: self.map,
            from: (*row, *col),
            steps: *steps + 1,
            uphill: self.uphill,
        })
    }
}

impl Problem for Day12 {
    type Input = RowsOfChars<char>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let start = input
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(col_idx, c)| if *c == START { Some(col_idx) } else { None })
                    .map(|col_idx| (row_idx, col_idx))
            })
            .expect("There should be a start position");

        let mut walker = PathFinder::new(input, END, true);
        walk::walk_broad(&mut walker, (0, start)).expect("There should be a path")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let start = input
            .iter()
            .enumerate()
            .find_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(col_idx, c)| if *c == END { Some(col_idx) } else { None })
                    .map(|col_idx| (row_idx, col_idx))
            })
            .expect("There should be a start position");

        let mut walker = PathFinder::new(input, 'a', false);
        walk::walk_broad(&mut walker, (0, start)).expect("There should be a path")
    }
}

fn main() {
    solve::<Day12>(include_str!("../../inputs/day12.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!(
        "Sabqponm\n",
        "abcryxxl\n",
        "accszExk\n",
        "acctuvwj\n",
        "abdefghi\n",
    );

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day12>(SAMPLE), 31);
        assert_eq!(solve_part2::<Day12>(SAMPLE), 29);
    }
}
