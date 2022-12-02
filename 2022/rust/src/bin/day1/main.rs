use advent_of_code::{get_day, read_file};
use itertools::sorted;

fn main() {
    let (p1, p2) = day1("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn day1(filename: &str) -> (u32, u32) {
    let input = read_file(get_day(file!()), filename);

    let elves = sorted(input.split("\n\n").map(|elf| {
        elf.split("\n")
            .map(|line| line.parse::<u32>().unwrap_or_default())
            .sum::<u32>()
    }));

    return (elves.clone().last().unwrap(), elves.rev().take(3).sum());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day1() {
        let (p1, p2) = crate::day1("sample.txt");
        assert_eq!(p1, 24000);
        assert_eq!(p2, 45000);
    }
}
