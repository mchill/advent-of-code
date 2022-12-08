use advent_of_code::{get_day, read_file};
use std::collections::VecDeque;

fn main() {
    let p1 = solve("input.txt", false);
    let p2 = solve("input.txt", true);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, multi_lift: bool) -> String {
    let input = read_file(get_day(file!()), filename);
    let (crates, instructions) = input.strip_suffix("\n").unwrap().split_once("\n\n").unwrap();

    let mut crates = crates.rsplit("\n").map(|line| line.chars().collect::<Vec<char>>());
    let instructions = instructions.split("\n");

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); (crates.next().unwrap().len() - 2) / 4 + 1];

    for line in crates {
        for i in (1..line.len()).step_by(4) {
            if line[i] != ' ' {
                stacks[(i - 1) / 4].push(line[i]);
            }
        }
    }

    for line in instructions {
        let line: Vec<&str> = line.split(" ").collect();
        let amount = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap();
        let to = line[5].parse::<usize>().unwrap();

        let mut moved: VecDeque<char> = VecDeque::new();
        for _ in 0..amount {
            moved.push_back(stacks[from - 1].pop().unwrap());
        }
        for _ in 0..amount {
            if multi_lift {
                stacks[to - 1].push(moved.pop_back().unwrap());
            } else {
                stacks[to - 1].push(moved.pop_front().unwrap());
            }
        }
    }

    return String::from_iter(stacks.iter().map(|stack| stack.last().unwrap()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let p1 = crate::solve("sample.txt", false);
        let p2 = crate::solve("sample.txt", true);
        assert_eq!(p1, "CMZ");
        assert_eq!(p2, "MCD");
    }
}
