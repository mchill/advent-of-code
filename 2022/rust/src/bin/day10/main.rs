use advent_of_code::{get_day, read_file};
use itertools::Itertools;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2:\n{}", p2);
}

fn solve(filename: &str) -> (i32, String) {
    let input = read_file(get_day(file!()), filename);
    let input = input.split_whitespace();

    let (mut p1, mut p2) = (0, String::new());
    let mut x = 1;

    for (cycle, line) in input.enumerate() {
        let cycle = cycle as i32 + 1;
        p2 += if i32::abs(x - (cycle - 1) % 40) <= 1 { "#" } else { "." };
        if (cycle - 20) % 40 == 0 {
            p1 += cycle * x;
        }
        x += line.parse::<i32>().unwrap_or(0);
    }

    return (p1, p2.chars().chunks(40).into_iter().map(|chunk| chunk.collect::<String>()).join("\n"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 13140);
        assert_eq!(
            p2,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
