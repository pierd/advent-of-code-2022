use std::collections::HashSet;

use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day09;

#[derive(Clone, Copy, Debug)]
#[rematch]
enum Direction {
    #[rematch(r"L")]
    Left,
    #[rematch(r"R")]
    Right,
    #[rematch(r"U")]
    Up,
    #[rematch(r"D")]
    Down,
}

impl Direction {
    fn coords(&self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[rematch(r"([LRUD]) (\d+)")]
struct Move {
    direction: Direction,
    count: isize,
}

#[derive(Clone, Debug)]
struct Rope<const SIZE: usize> {
    sections: [(isize, isize); SIZE],
    tail_positions: HashSet<(isize, isize)>,
}

impl<const SIZE: usize> Default for Rope<SIZE>
where
    [(isize, isize); SIZE]: Default,
{
    fn default() -> Self {
        let mut tail_positions = HashSet::new();
        tail_positions.insert(Default::default());
        Self {
            sections: Default::default(),
            tail_positions,
        }
    }
}

impl<const SIZE: usize> Rope<SIZE> {
    fn apply_move(&mut self, mv: &Move) {
        let (dx, dy) = mv.direction.coords();
        for _ in 0..mv.count {
            self.sections[0].0 += dx;
            self.sections[0].1 += dy;
            self.fix_tail();
        }
    }

    fn fix_tail(&mut self) {
        for tail_idx in 1..SIZE {
            let head_idx = tail_idx - 1;
            let dx = self.sections[head_idx].0 - self.sections[tail_idx].0;
            let dy = self.sections[head_idx].1 - self.sections[tail_idx].1;
            if dx.abs() > 1 || dy.abs() > 1 {
                self.sections[tail_idx].0 += dx.signum();
                self.sections[tail_idx].1 += dy.signum();
            }
        }
        self.tail_positions.insert(self.sections[SIZE - 1]);
    }
}

impl Problem for Day09 {
    type Input = VecFromLines<Move>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut r = Rope::<2>::default();
        for m in input {
            r.apply_move(m);
        }
        r.tail_positions.len()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut r = Rope::<10>::default();
        for m in input {
            r.apply_move(m);
        }
        r.tail_positions.len()
    }
}

fn main() {
    solve::<Day09>(include_str!("../../inputs/day09.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    const SAMPLE2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day09>(SAMPLE), 13);
        assert_eq!(solve_part2::<Day09>(SAMPLE2), 36);
    }
}
