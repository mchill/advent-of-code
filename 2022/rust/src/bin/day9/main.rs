use std::collections::HashSet;

use advent_of_code::{get_day, read_lines};

fn main() {
    let p1 = solve("input.txt", 2);
    let p2 = solve("input.txt", 10);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, length: usize) -> usize {
    let input = read_lines(get_day(file!()), filename);

    let mut knots = vec![(0, 0); length];
    let mut visited = HashSet::from([(0, 0)]);

    for line in input {
        let (dir, distance) = line.split_once(" ").unwrap();
        let distance = distance.parse::<u32>().unwrap();

        for _ in 0..distance {
            match dir {
                "R" => knots[0].0 += 1,
                "L" => knots[0].0 -= 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => unreachable!(),
            }

            for i in 1..knots.len() {
                let leader = knots[i - 1];
                let mut follower = &mut knots[i];

                let x_diff: i32 = leader.0 - follower.0;
                let y_diff: i32 = leader.1 - follower.1;

                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    follower.0 += x_diff.clamp(-1, 1);
                    follower.1 += y_diff.clamp(-1, 1);
                }
            }

            visited.insert(knots.last().unwrap().clone());
        }
    }

    return visited.len();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let p1 = crate::solve("sample1.txt", 2);
        let p2 = crate::solve("sample2.txt", 10);
        assert_eq!(p1, 13);
        assert_eq!(p2, 36);
    }
}
