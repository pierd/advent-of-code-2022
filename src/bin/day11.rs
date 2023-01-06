use std::collections::VecDeque;

use aoc_helpers::{prelude::*, scaffold::Parse};
use rematch::rematch;

struct Day11;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[rematch]
enum Operation {
    #[rematch(r"\+")]
    Add,
    #[rematch(r"\*")]
    Multiply,
}

impl Operation {
    fn perform(self, operand1: usize, operand2: usize) -> usize {
        match self {
            Operation::Add => operand1 + operand2,
            Operation::Multiply => operand1 * operand2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[rematch]
enum Operand {
    #[rematch(r"old")]
    Old,
    #[rematch(r"(\d+)")]
    Const(usize),
}

impl Operand {
    fn eval(self, old: usize) -> usize {
        match self {
            Operand::Old => old,
            Operand::Const(v) => v,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Monkey {
    idx: usize,
    items: VecDeque<usize>,
    operation: Operation,
    operands: [Operand; 2],
    test: usize,
    true_target: usize,
    false_target: usize,
    inspections: usize,
}

impl Parse for Monkey {
    type Parsed = Self;

    fn parse(raw_input: &str) -> anyhow::Result<Self::Parsed> {
        lazy_static::lazy_static! {
            static ref PATTERN: regex::Regex = regex::Regex::new(concat!(
                r"Monkey (\d+):\n",
                r"  Starting items: ([\d, ]+)\n",
                // r"  Operation: new = ([old\d+* ]+)\n",
                r"  Operation: new = (.+)\n",
                r"  Test: divisible by (\d+)\n",
                r"    If true: throw to monkey (\d+)\n",
                r"    If false: throw to monkey (\d+)",
            )).unwrap();
        }

        if let Some(caps) = PATTERN.captures(raw_input.trim()) {
            let raw_operation: Vec<&str> = caps
                .get(3)
                .ok_or_else(|| anyhow::anyhow!("Getting idx failed"))?
                .as_str()
                .split_whitespace()
                .take(3)
                .collect();
            return Ok(Self {
                idx: caps
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("Getting idx failed"))?
                    .as_str()
                    .parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Field 'idx' parsing error: {}", e))?,
                items: caps
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("Getting items failed"))?
                    .as_str()
                    .split(", ")
                    .map(|int_str| {
                        int_str
                            .parse::<usize>()
                            .map_err(|e| anyhow::anyhow!("Item parsing failed: {}", e))
                    })
                    .collect::<Result<VecDeque<usize>, _>>()?,
                operation: raw_operation[1]
                    .parse::<Operation>()
                    .map_err(|e| anyhow::anyhow!("Operation parsing error: {}", e))?,
                operands: [
                    raw_operation[0]
                        .parse::<Operand>()
                        .map_err(|e| anyhow::anyhow!("Operand parsing error: {}", e))?,
                    raw_operation[2]
                        .parse::<Operand>()
                        .map_err(|e| anyhow::anyhow!("Operand parsing error: {}", e))?,
                ],
                test: caps
                    .get(4)
                    .ok_or_else(|| anyhow::anyhow!("Getting test failed"))?
                    .as_str()
                    .parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Field 'test' parsing error: {}", e))?,
                true_target: caps
                    .get(5)
                    .ok_or_else(|| anyhow::anyhow!("Getting true condition failed"))?
                    .as_str()
                    .parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Field 'true condition' parsing error: {}", e))?,
                false_target: caps
                    .get(6)
                    .ok_or_else(|| anyhow::anyhow!("Getting false condition failed"))?
                    .as_str()
                    .parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Field 'false condition' parsing error: {}", e))?,
                inspections: 0,
            });
        }

        Err(anyhow::anyhow!("Can't parse: {:?}", raw_input))
    }
}

struct InspectionResult {
    target_monkey: usize,
    item: usize,
}

impl Monkey {
    fn inspect(&mut self, div: usize, modulo: usize) -> Option<InspectionResult> {
        if let Some(item) = self.items.pop_front() {
            self.inspections += 1;
            let new_item = self
                .operation
                .perform(self.operands[0].eval(item), self.operands[1].eval(item))
                / div
                % modulo;
            Some(InspectionResult {
                target_monkey: if new_item % self.test == 0 {
                    self.true_target
                } else {
                    self.false_target
                },
                item: new_item,
            })
        } else {
            None
        }
    }

    fn perform_round(monkeys: &mut Vec<Self>, div: usize, modulo: usize) {
        for idx in 0..monkeys.len() {
            while let Some(InspectionResult {
                target_monkey,
                item,
            }) = monkeys[idx].inspect(div, modulo)
            {
                monkeys[target_monkey].items.push_back(item);
            }
        }
    }
}

impl Problem for Day11 {
    type Input = VecFromMultiLines<Monkey>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut monkeys = input.clone();
        let modulo = monkeys.iter().map(|m| m.test).product();
        for _ in 0..20 {
            Monkey::perform_round(&mut monkeys, 3, modulo);
        }
        let mut inspection_counts: Vec<usize> =
            monkeys.into_iter().map(|m| m.inspections).collect();
        inspection_counts.sort_unstable();
        inspection_counts.reverse();
        inspection_counts[0] * inspection_counts[1]
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut monkeys = input.clone();
        let modulo = monkeys.iter().map(|m| m.test).product();
        for _ in 0..10000 {
            Monkey::perform_round(&mut monkeys, 1, modulo);
        }
        let mut inspection_counts: Vec<usize> =
            monkeys.into_iter().map(|m| m.inspections).collect();
        inspection_counts.sort_unstable();
        inspection_counts.reverse();
        inspection_counts[0] * inspection_counts[1]
    }
}

fn main() {
    solve::<Day11>(include_str!("../../inputs/day11.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!(
        "Monkey 0:\n",
        "  Starting items: 79, 98\n",
        "  Operation: new = old * 19\n",
        "  Test: divisible by 23\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 1:\n",
        "  Starting items: 54, 65, 75, 74\n",
        "  Operation: new = old + 6\n",
        "  Test: divisible by 19\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 0\n",
        "\n",
        "Monkey 2:\n",
        "  Starting items: 79, 60, 97\n",
        "  Operation: new = old * old\n",
        "  Test: divisible by 13\n",
        "    If true: throw to monkey 1\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 3:\n",
        "  Starting items: 74\n",
        "  Operation: new = old + 3\n",
        "  Test: divisible by 17\n",
        "    If true: throw to monkey 0\n",
        "    If false: throw to monkey 1\n",
    );

    #[test]
    fn test_monkey_parsing() {
        let monkeys = VecFromMultiLines::<Monkey>::parse(SAMPLE).unwrap();
        assert_eq!(
            monkeys[0],
            Monkey {
                idx: 0,
                items: vec![79, 98].into(),
                operation: Operation::Multiply,
                operands: [Operand::Old, Operand::Const(19)],
                test: 23,
                true_target: 2,
                false_target: 3,
                inspections: 0,
            }
        );
    }

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day11>(SAMPLE), 10605);
        assert_eq!(solve_part2::<Day11>(SAMPLE), 2713310158);
    }
}
