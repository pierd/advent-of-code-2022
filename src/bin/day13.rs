use std::{cmp::Ordering, fmt::{Display, Write}};

use aoc_helpers::{prelude::*, scaffold::Parse};

struct Day13;

enum Packet {
    List(Vec<Packet>),
    Num(usize),
}

impl Packet {
    fn list_from_str_iter<'a, I>(mut iter: I) -> (Option<Self>, I)
    where
        I: Iterator<Item = &'a str>,
    {
        let mut result = Vec::new();
        while let Some(i) = iter.next() {
            match i {
                "," => continue,
                "]" => break,
                "[" => {
                    let (list, new_iter) = Self::list_from_str_iter(iter);
                    result.push(list.unwrap());
                    iter = new_iter;
                }
                s => result.push(Self::Num(s.parse().unwrap())),
            }
        }
        (Some(Self::List(result)), iter)
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => {
                let mut a_iter = a.iter();
                let mut b_iter = b.iter();
                loop {
                    match (a_iter.next(), b_iter.next()) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return  Ordering::Greater,
                        (Some(a), Some(b)) => {
                            let result = Self::compare(a, b);
                            if result != Ordering::Equal {
                                return result;
                            }
                        }
                    }
                }
            },
            (Packet::List(a), Packet::Num(b)) => {
                if let Some(a_first) = a.first() {
                    let result = Self::compare(a_first, &Packet::Num(*b));
                    if result != Ordering::Equal {
                        return result;
                    }
                    if a.len() > 1 {
                        return Ordering::Greater;
                    }
                }
                Ordering::Less
            }
            (Packet::Num(a), Packet::List(b)) => {
                if let Some(b_first) = b.first() {
                    let result = Self::compare(&Packet::Num(*a), b_first);
                    if result != Ordering::Equal {
                        return result;
                    }
                    if b.len() > 1 {
                        return Ordering::Less;
                    }
                }
                Ordering::Greater
            }
            (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
        }
    }
}

impl Parse for Packet {
    type Parsed = Self;

    fn parse(raw_input: &str) -> anyhow::Result<Self::Parsed> {
        lazy_static::lazy_static! {
            static ref PATTERN: regex::Regex = regex::Regex::new(r"\[|\]|,|\d+").unwrap();
        }

        let mut iter = PATTERN.find_iter(raw_input.trim()).map(|m| m.as_str());
        let open = iter.next();
        assert_eq!(open, Some("["));
        Self::list_from_str_iter(iter)
            .0
            .ok_or_else(|| anyhow::anyhow!("Failed to parse: {:?}", raw_input))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(l) => {
                f.write_char('[')?;
                if let Some((first, rest)) = l.split_first() {
                    first.fmt(f)?;
                    for p in rest {
                        f.write_char(',')?;
                        p.fmt(f)?;
                    }
                }
                f.write_char(']')?;
                Ok(())
            }
            Packet::Num(n) => n.fmt(f)
        }
    }
}

impl Problem for Day13 {
    type Input = VecFromMultiLines<VecFromLines<Packet>>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        input.iter().enumerate().filter_map(|(idx, packets)| {
            if Packet::compare(&packets[0], &packets[1]) == Ordering::Less {
                Some(idx + 1)
            } else {
                None
            }
        }).sum()
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        Default::default()
    }
}

fn main() {
    solve::<Day13>(include_str!("../../inputs/day13.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!(
        "[1,1,3,1,1]\n",
        "[1,1,5,1,1]\n",
        "\n",
        "[[1],[2,3,4]]\n",
        "[[1],4]\n",
        "\n",
        "[9]\n",
        "[[8,7,6]]\n",
        "\n",
        "[[4,4],4,4]\n",
        "[[4,4],4,4,4]\n",
        "\n",
        "[7,7,7,7]\n",
        "[7,7,7]\n",
        "\n",
        "[]\n",
        "[3]\n",
        "\n",
        "[[[]]]\n",
        "[[]]\n",
        "\n",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]\n",
    );

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day13>(SAMPLE), 13);
        assert_eq!(solve_part2::<Day13>(SAMPLE), Default::default());
    }

    #[test]
    fn test_parsing_and_display() {
        for l in SAMPLE.lines() {
            if !l.is_empty() {
                let packet = Packet::parse(l).unwrap();
                assert_eq!(l, format!("{}", packet));
            }
        }
    }
}
