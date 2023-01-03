use aoc_helpers::prelude::*;

struct Day08;

#[derive(Clone, Copy, Debug, Default)]
struct Tree {
    height: usize,
    visible: bool,
}

impl TryFrom<char> for Tree {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0'..='9' => Ok(Self {
                height: (value as u8 - b'0') as usize,
                ..Default::default()
            }),
            _ => Err(anyhow::anyhow!("Can't parse: {:?}", value)),
        }
    }
}

fn mark_visible<'a>(mut iter: impl Iterator<Item = &'a mut Tree>) {
    let mut highest = iter.next().unwrap().height;
    for tree in iter {
        if highest < tree.height {
            tree.visible = true;
            highest = tree.height;
        }
    }
}

fn count_distance<'a>(mut iter: impl Iterator<Item = &'a Tree>) -> usize {
    let height = iter.next().unwrap().height;
    let mut distance = 0;
    for tree in iter {
        distance += 1;
        if tree.height >= height {
            break;
        }
    }
    distance
}

impl Problem for Day08 {
    type Input = RowsOfChars<Tree>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut forrest = input.clone();

        // mark edges as visible
        for t in &mut forrest[0] {
            t.visible = true;
        }
        for t in forrest.last_mut().expect("There should be trees") {
            t.visible = true;
        }
        for row in &mut forrest {
            row[0].visible = true;
            row.last_mut().expect("There should be trees").visible = true;
        }

        // solve
        for row in &mut forrest {
            mark_visible(row.iter_mut());
            mark_visible(row.iter_mut().rev());
        }
        for column in 1..forrest[0].len() {
            mark_visible(forrest.iter_mut().map(|row| row.get_mut(column).unwrap()));
            mark_visible(
                forrest
                    .iter_mut()
                    .rev()
                    .map(|row| row.get_mut(column).unwrap()),
            );
        }

        // count visible
        forrest
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|t| t.visible)
            .count()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let rows = input.len();
        let cols = input[0].len();
        let mut best_scenic_score = 0;
        for row in 1..rows {
            for col in 1..cols {
                let right = count_distance(input[row].iter().skip(col));
                let left = count_distance(input[row].iter().rev().skip(cols - col - 1));
                let down = count_distance(input.iter().map(|row| &row[col]).skip(row));
                let up =
                    count_distance(input.iter().rev().map(|row| &row[col]).skip(rows - row - 1));
                let scenic_score = right * left * down * up;
                if best_scenic_score < scenic_score {
                    best_scenic_score = scenic_score;
                }
            }
        }
        best_scenic_score
    }
}

fn main() {
    solve::<Day08>(include_str!("../../inputs/day08.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390\n",);

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day08>(SAMPLE), 21);
        assert_eq!(solve_part2::<Day08>(SAMPLE), 8);
    }
}
