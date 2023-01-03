use aoc_helpers::{interpret::Execute, prelude::*};
use rematch::rematch;

struct Day10;

#[derive(Clone, Copy, Debug)]
#[rematch]
enum Command {
    #[rematch(r"addx (-?\d+)")]
    Add(isize),
    #[rematch(r"noop")]
    Noop,
}

const SPECIAL_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Clone, Copy, Debug)]
struct State {
    cycle: usize,
    register: isize,
    // 20th, 60th, 100th, 140th, 180th, and 220th
    cycle_values: [Option<isize>; 6],
}

impl Default for State {
    fn default() -> Self {
        Self {
            cycle: 1,
            register: 1,
            cycle_values: Default::default(),
        }
    }
}

impl Execute<State> for Command {
    fn execute(&self, mut state: State) -> (State, interpret::Jump) {
        state.cycle += 1;
        for (cycle, special) in state
            .cycle_values
            .iter_mut()
            .zip(SPECIAL_CYCLES.into_iter())
        {
            if state.cycle == special {
                *cycle = Some(special as isize * state.register);
            }
        }

        if let Command::Add(x) = self {
            state.register += *x;
            state.cycle += 1;

            for (cycle, special) in state
                .cycle_values
                .iter_mut()
                .zip(SPECIAL_CYCLES.into_iter())
            {
                if state.cycle == special {
                    *cycle = Some(special as isize * state.register);
                }
            }
        }

        (
            state,
            if state.cycle_values.last().unwrap().is_some() {
                interpret::Jump::Stop
            } else {
                Default::default()
            },
        )
    }
}

#[derive(Clone, Debug)]
struct DrawingState {
    register: isize,
    pixels: Vec<bool>,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            register: 1,
            pixels: Default::default(),
        }
    }
}

impl Execute<DrawingState> for Command {
    fn execute(&self, mut state: DrawingState) -> (DrawingState, interpret::Jump) {
        let offset = state.register;
        let crt = (state.pixels.len() % 40) as isize;
        state
            .pixels
            .push(offset == crt || offset - 1 == crt || offset + 1 == crt);

        if let Command::Add(x) = self {
            state.register += *x;
            let crt = (state.pixels.len() % 40) as isize;
            state
                .pixels
                .push(offset == crt || offset - 1 == crt || offset + 1 == crt);
        }

        let cycles = state.pixels.len();
        (
            state,
            if cycles >= 240 {
                interpret::Jump::Stop
            } else {
                Default::default()
            },
        )
    }
}

impl Problem for Day10 {
    type Input = VecFromLines<Command>;
    type Part1 = isize;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let state: State = input.execute(Default::default()).0;
        assert!(state.cycle_values.iter().all(Option::is_some));
        state.cycle_values.into_iter().map(Option::unwrap).sum()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let state: DrawingState = input.execute(Default::default()).0;
        assert!(state.pixels.len() >= 240);
        let mut result = String::new();
        result.push('\n');
        for row in state.pixels.chunks(40).into_iter().map(|chunk| {
            chunk
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        }) {
            result.push_str(&row);
            result.push('\n');
        }
        result
    }
}

fn main() {
    solve::<Day10>(include_str!("../../inputs/day10.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = include_str!("../../inputs/day10-sample.txt");

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day10>(SAMPLE), 13140);
        assert_eq!(
            solve_part2::<Day10>(SAMPLE),
            concat!(
                "\n",
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######.....\n",
            )
        );
    }
}
