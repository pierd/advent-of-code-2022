use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day02;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[rematch]
enum Shape {
    #[rematch(r"A|X")]
    Rock,
    #[rematch(r"B|Y")]
    Paper,
    #[rematch(r"C|Z")]
    Scissors,
}

impl Shape {
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn winner(&self) -> Self {
        match self {
            Self::Scissors => Self::Rock,
            Self::Paper => Self::Scissors,
            Self::Rock => Self::Paper,
        }
    }

    fn loser(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
            Self::Paper => Self::Rock,
        }
    }

    fn defeats(&self, other: &Self) -> bool {
        other.winner() == *self
    }

    fn expected(&self, result: &RoundResult) -> Self {
        match result {
            RoundResult::Lose => self.loser(),
            RoundResult::Draw => *self,
            RoundResult::Win => self.winner(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[rematch(r"([ABC]) ([XYZ])")]
struct Round {
    opponent: Shape,
    player: Shape,
}

impl Round {
    fn score(&self) -> usize {
        self.outcome_score() + self.player.score()
    }

    fn outcome_score(&self) -> usize {
        if self.opponent.defeats(&self.player) {
            0
        } else if self.player.defeats(&self.opponent) {
            6
        } else {
            3
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl From<&Shape> for RoundResult {
    fn from(s: &Shape) -> RoundResult {
        match s {
            Shape::Rock => Self::Lose,
            Shape::Paper => Self::Draw,
            Shape::Scissors => Self::Win,
        }
    }
}

impl RoundResult {
    fn score(&self) -> usize {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

struct OutcomeRound {
    opponent: Shape,
    result: RoundResult,
}

impl From<&Round> for OutcomeRound {
    fn from(s: &Round) -> OutcomeRound {
        OutcomeRound {
            opponent: s.opponent,
            result: (&s.player).into(),
        }
    }
}

impl OutcomeRound {
    fn score(&self) -> usize {
        self.opponent.expected(&self.result).score() + self.result.score()
    }
}

impl Problem for Day02 {
    type Input = VecFromLines<Round>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.iter().map(Round::score).sum()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        input
            .iter()
            .map(|round| Into::<OutcomeRound>::into(round).score())
            .sum()
    }
}

fn main() {
    solve::<Day02>(include_str!("../../inputs/day02.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day02>(SAMPLE), 15);
        assert_eq!(solve_part2::<Day02>(SAMPLE), 12);
    }
}
