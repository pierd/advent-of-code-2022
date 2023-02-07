use std::collections::HashMap;

use aoc_helpers::prelude::*;
use rematch::rematch;

const SIZE: usize = 70000000;
const NEEDED: usize = 30000000;

struct Day07;

#[derive(Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
#[rematch]
enum CLI {
    #[rematch(r"\$ cd /")]
    ChangeDirRoot,
    #[rematch(r"\$ cd \.\.")]
    ChangeDirUp,
    #[rematch(r"\$ cd ([a-z.]+)")]
    ChangeDir(String),
    #[rematch(r"\$ ls")]
    List,
    #[rematch(r"dir ([a-z.]+)")]
    Directory { name: String },
    #[rematch(r"(\d+) ([a-z.]+)")]
    File { size: usize, name: String },
}

#[derive(Clone, Debug)]
enum FS {
    File(usize),
    Directory(HashMap<String, FS>),
}

impl Default for FS {
    fn default() -> Self {
        Self::Directory(Default::default())
    }
}

impl FS {
    fn from_cli_lines(lines: &[CLI]) -> Option<Self> {
        if let Some((CLI::ChangeDirRoot, rest)) = lines.split_first() {
            let mut root = Default::default();
            if FS::interpret(&mut root, rest)
                .map(|leftover| leftover.is_empty())
                .unwrap_or_default()
            {
                return Some(FS::Directory(root));
            }
        }
        None
    }

    fn interpret<'a>(dir: &mut HashMap<String, Self>, mut lines: &'a [CLI]) -> Option<&'a [CLI]> {
        while let Some((first, rest)) = lines.split_first() {
            if let Some(new_lines) = match first {
                CLI::ChangeDirRoot => {
                    unreachable!();
                }
                CLI::ChangeDirUp => {
                    return Some(rest);
                }
                CLI::ChangeDir(dir_name) => match dir.get_mut(dir_name) {
                    Some(FS::Directory(subdir)) => FS::interpret(subdir, rest),
                    _ => return None,
                },
                CLI::List => Some(rest), // no-op
                CLI::Directory { name } => {
                    dir.insert(name.clone(), Default::default());
                    Some(rest)
                }

                CLI::File { size, name } => {
                    dir.insert(name.clone(), FS::File(*size));
                    Some(rest)
                }
            } {
                lines = new_lines;
            } else {
                return None;
            }
        }
        Some(lines)
    }

    fn size_walk<F: FnMut(&Self, usize)>(&self, fun: &mut F) -> usize {
        let size = match self {
            FS::Directory(dir) => dir.iter().map(|(_, fs)| fs.size_walk(fun)).sum(),
            FS::File(size) => *size,
        };
        fun(self, size);
        size
    }
}

impl Problem for Day07 {
    type Input = VecFromLines<CLI>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let fs = FS::from_cli_lines(input).expect("Input should be valid");
        let mut total_size = 0;
        fs.size_walk(&mut |entry, size| {
            if let FS::Directory(_) = entry {
                if size <= 100000 {
                    total_size += size;
                }
            }
        });
        total_size
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let fs = FS::from_cli_lines(input).expect("Input should be valid");
        let mut dir_sizes = Vec::new();
        let total_size = fs.size_walk(&mut |entry, size| {
            if let FS::Directory(_) = entry {
                dir_sizes.push(size);
            }
        });
        let size_needed = NEEDED - (SIZE - total_size);
        let mut best_size = total_size;
        for dir_size in dir_sizes {
            if dir_size >= size_needed && dir_size < best_size {
                best_size = dir_size;
            }
        }
        best_size
    }
}

fn main() {
    solve::<Day07>(include_str!("../../inputs/day07.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ ls\n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k\n",
    );

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day07>(SAMPLE), 95437);
        assert_eq!(solve_part2::<Day07>(SAMPLE), 24933642);
    }
}
