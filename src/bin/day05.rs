use aoc_helpers::prelude::*;
use rematch::rematch;

struct Day05;

#[derive(Clone, Debug)]
struct Stacks(Vec<Vec<char>>);

impl aoc_helpers::scaffold::Parse for Stacks {
    type Parsed = Self;
    fn parse(raw_input: &str) -> anyhow::Result<Self::Parsed> {
        let mut lines = raw_input
            .lines()
            .map(|line| line.chars())
            .collect::<Vec<_>>();

        let mut last_line = lines
            .pop()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse last line of stacks"))?;
        let mut expect_char_in_last_line = |chr| {
            last_line
                .next()
                .map(|c| c == chr)
                .ok_or_else(|| anyhow::anyhow!("Failed to parse last line of stacks"))
        };
        expect_char_in_last_line(' ')?;
        expect_char_in_last_line('1')?;
        expect_char_in_last_line(' ')?;

        // drop the first character of each line
        for line in &mut lines {
            line.next()
                .ok_or_else(|| anyhow::anyhow!("Failed to trim first char of stacks"))?;
        }

        let mut result = Vec::new();
        let mut finished = false;
        while !finished {
            let mut stack = Vec::new();
            let mut finished_count = 0;
            for line in lines.iter_mut().rev() {
                if let Some(c) = line.next() {
                    if c != ' ' {
                        stack.push(c);
                    }
                }
            }
            result.push(stack);

            // drop another 3 characters of each line
            for line in &mut lines {
                let mut line_finished = false;
                for _ in 0..3 {
                    if line.next().is_none() {
                        line_finished = true;
                    }
                }
                if line_finished {
                    finished_count += 1;
                }
            }
            finished = finished_count == lines.len();
        }

        Ok(Self(result))
    }
}

#[derive(Clone, Copy, Debug)]
#[rematch(r"move (\d+) from (\d+) to (\d+)")]
struct Command {
    quantity: usize,
    source: usize,
    target: usize,
}

impl Problem for Day05 {
    type Input = TwoSections<Stacks, VecFromLines<Command>>;
    type Part1 = String;
    type Part2 = String;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut stacks = input.0 .0.clone();
        for Command {
            quantity,
            source,
            target,
        } in &input.1
        {
            for _ in 0..*quantity {
                if let Some(c) = stacks[*source - 1].pop() {
                    stacks[*target - 1].push(c);
                }
            }
        }
        stacks
            .into_iter()
            .map(|mut s| s.pop().unwrap_or(' '))
            .collect()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut stacks = input.0 .0.clone();
        for Command {
            quantity,
            source,
            target,
        } in &input.1
        {
            if source == target {
                continue;
            }

            let mut target_vec = Vec::new();
            let mut source_vec = Vec::new();

            std::mem::swap(&mut target_vec, &mut stacks[*target - 1]);
            std::mem::swap(&mut source_vec, &mut stacks[*source - 1]);

            target_vec.extend(source_vec.drain((source_vec.len() - *quantity)..));

            std::mem::swap(&mut target_vec, &mut stacks[*target - 1]);
            std::mem::swap(&mut source_vec, &mut stacks[*source - 1]);
        }
        stacks
            .into_iter()
            .map(|mut s| s.pop().unwrap_or(' '))
            .collect()
    }
}

fn main() {
    solve::<Day05>(include_str!("../../inputs/day05.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2, Parse};

    const SAMPLE: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n",
    );

    #[test]
    fn test_stacks_parsing() {
        let stacks_str = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 "
        );
        let stacks = Stacks::parse(stacks_str).unwrap();
        assert_eq!(
            stacks.0,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day05>(SAMPLE), "CMZ".to_owned());
        assert_eq!(solve_part2::<Day05>(SAMPLE), "MCD".to_owned());
    }
}
