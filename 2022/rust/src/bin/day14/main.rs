use advent_of_code::{get_day, read_lines};
use itertools::Itertools;
use std::{cmp::max, cmp::min, collections::HashSet};

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);
    let (mut p1, mut p2) = (0, 0);

    let mut cave: HashSet<(i32, i32)> = HashSet::new();

    for line in input {
        let vertices = line.split(" -> ").map(|vertex| {
            vertex
                .split_once(",")
                .map(|vertex| (vertex.0.parse().unwrap(), vertex.1.parse().unwrap()))
                .unwrap()
        });

        for (v1, v2) in vertices.tuple_windows() {
            for x in min(v1.0, v2.0)..max(v1.0, v2.0) + 1 {
                for y in min(v1.1, v2.1)..max(v1.1, v2.1) + 1 {
                    cave.insert((x, y));
                }
            }
        }
    }

    let start = (500, 0);
    let max_depth = cave.iter().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1;
    for x in start.0 - max_depth - 2..start.0 + max_depth + 3 {
        cave.insert((x, max_depth + 2));
    }

    while !cave.contains(&start) {
        let mut sand = start.clone();
        loop {
            if sand.1 >= max_depth && p1 == 0 {
                p1 = p2;
            }
            if !cave.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !cave.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !cave.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                cave.insert(sand);
                p2 += 1;
                break;
            }
        }
    }

    return (p1, p2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 24);
        assert_eq!(p2, 93);
    }
}
