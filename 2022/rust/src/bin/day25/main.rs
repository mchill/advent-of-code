use advent_of_code::{get_day, read_lines};

fn main() {
    let p1 = solve("input.txt");
    println!("Part 1: {}", p1);
}

fn solve(filename: &str) -> String {
    let input = read_lines(get_day(file!()), filename);

    let mut sum = 0;
    for line in input {
        for (i, c) in line.chars().rev().enumerate() {
            sum += 5_i64.pow(i as u32) * ("=-012".chars().position(|ch| ch == c).unwrap() as i64 - 2);
        }
    }

    let mut p1 = "".to_owned();
    while sum > 0 {
        p1.insert(0, "012=-".chars().nth(sum as usize % 5).unwrap());
        sum = (sum + 2) / 5;
    }

    return p1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let p1 = crate::solve("sample.txt");
        assert_eq!(p1, "2=-1=0");
    }
}
