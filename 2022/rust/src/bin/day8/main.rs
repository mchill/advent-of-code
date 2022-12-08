use advent_of_code::{get_day, read_lines};
use std::cmp::max;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);
    let forest: &Vec<Vec<u32>> = &input.iter().map(|row| row.chars().map(|char| char.to_digit(10).unwrap()).collect()).collect();
    let (mut p1, mut p2) = (0, 0);

    for y in 0..forest.len() {
        let y_max = forest.len();
        let x_max = forest[y].len();

        for x in 0..x_max {
            let ranges = get_ranges((x_max, y_max), (x, y));

            let is_visible = ranges
                .iter()
                .fold(false, |acc, range| acc || is_visible_from_edge(forest, (x, y), &range.0, &range.1));
            if is_visible {
                p1 += 1;
            }

            let score = ranges
                .iter()
                .fold(1, |acc, range| acc * count_visible_from_position(forest, (x, y), &range.0, &range.1));
            p2 = max(p2, score)
        }
    }

    return (p1, p2);
}

fn get_ranges(size: (usize, usize), position: (usize, usize)) -> Vec<(Vec<usize>, Vec<usize>)> {
    return vec![
        ((0..position.0).rev().collect(), (position.1..position.1 + 1).collect()),
        ((position.0 + 1..size.0).collect(), (position.1..position.1 + 1).collect()),
        ((position.0..position.0 + 1).collect(), (0..position.1).rev().collect()),
        ((position.0..position.0 + 1).collect(), (position.1 + 1..size.1).collect()),
    ];
}

fn is_visible_from_edge(forest: &Vec<Vec<u32>>, position: (usize, usize), x_range: &Vec<usize>, y_range: &Vec<usize>) -> bool {
    for y in y_range {
        for x in x_range {
            if forest[*y][*x] >= forest[position.1][position.0] {
                return false;
            }
        }
    }
    return true;
}

fn count_visible_from_position(forest: &Vec<Vec<u32>>, position: (usize, usize), x_range: &Vec<usize>, y_range: &Vec<usize>) -> u32 {
    let mut visible = 0;
    for y in y_range {
        for x in x_range {
            visible += 1;
            if forest[*y][*x] >= forest[position.1][position.0] {
                return visible;
            }
        }
    }
    return visible;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day8() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 21);
        assert_eq!(p2, 8);
    }
}
