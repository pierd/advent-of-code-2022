use aoc_helpers::prelude::*;

struct Day06;

fn find_first_unique(bytes: &[u8], size: usize) -> Option<usize> {
    bytes
        .windows(size)
        .enumerate()
        .find(|(_, chars)| {
            chars
                .iter()
                .enumerate()
                .all(|(i, c)| chars.iter().skip(i + 1).all(|other| *c != *other))
        })
        .map(|(idx, _)| idx + size)
}

impl Problem for Day06 {
    type Input = String;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        find_first_unique(input.as_bytes(), 4).expect("There should be a solution")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        find_first_unique(input.as_bytes(), 14).expect("There should be a solution")
    }
}

fn main() {
    solve::<Day06>(include_str!("../../inputs/day06.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day06>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1::<Day06>("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(
            solve_part1::<Day06>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(solve_part1::<Day06>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(solve_part2::<Day06>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2::<Day06>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2::<Day06>("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(
            solve_part2::<Day06>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
        assert_eq!(solve_part2::<Day06>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
