use advent_of_code::{get_day, read_lines};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);

    let (mut p1, mut p2) = (0, 0);

    for line in input.clone() {
        let mut first: HashSet<char> = HashSet::from_iter(line[..line.len() / 2].chars());
        let second: HashSet<char> = HashSet::from_iter(line[line.len() / 2..].chars());

        first.retain(|x| second.contains(x));
        p1 += get_priority(*first.iter().next().unwrap());
    }

    for i in (0..input.len()).step_by(3) {
        let mut first: HashSet<char> = HashSet::from_iter(input[i].chars());
        let second: HashSet<char> = HashSet::from_iter(input[i + 1].chars());
        let third: HashSet<char> = HashSet::from_iter(input[i + 2].chars());

        first.retain(|x| second.contains(x) && third.contains(x));
        p2 += get_priority(*first.iter().next().unwrap());
    }

    return (p1, p2);
}

fn get_priority(item: char) -> u32 {
    return if item.is_ascii_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 157);
        assert_eq!(p2, 70);
    }
}
