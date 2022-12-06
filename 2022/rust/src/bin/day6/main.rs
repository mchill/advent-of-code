use advent_of_code::{get_day, read_file};
use std::collections::HashSet;

fn main() {
    let p1 = solve("input.txt", 4);
    let p2 = solve("input.txt", 14);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, distinct_size: usize) -> usize {
    let input = read_file(get_day(file!()), filename);

    for index in distinct_size..input.len() {
        if input[index - distinct_size..index].chars().collect::<HashSet<_>>().len() == distinct_size {
            return index;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day6() {
        let p1 = crate::solve("sample.txt", 4);
        let p2 = crate::solve("sample.txt", 14);
        assert_eq!(p1, 7);
        assert_eq!(p2, 19);
    }
}
