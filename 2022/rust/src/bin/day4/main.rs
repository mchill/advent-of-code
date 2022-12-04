use advent_of_code::{get_day, read_lines};

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);
    let (mut p1, mut p2) = (0, 0);

    for line in input {
        let mut elves = line.split(",").map(|elf| elf.split("-").map(|x| x.parse::<u32>().unwrap()).collect());
        let elf1: Vec<u32> = elves.next().unwrap();
        let elf2: Vec<u32> = elves.next().unwrap();

        if elf1[0] <= elf2[0] && elf1[1] >= elf2[1] || elf2[0] <= elf1[0] && elf2[1] >= elf1[1] {
            p1 += 1;
            p2 += 1;
        } else if elf1[0] >= elf2[0] && elf1[0] <= elf2[1] || elf1[1] >= elf2[0] && elf1[1] <= elf2[1] {
            p2 += 1;
        }
    }

    return (p1, p2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day4() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 2);
        assert_eq!(p2, 4);
    }
}
