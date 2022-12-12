use advent_of_code::{get_day, read_lines};
use std::collections::{HashSet, VecDeque};

fn main() {
    let p1 = solve("input.txt", 'S', 'E', |from, to| to <= from + 1);
    let p2 = solve("input.txt", 'E', 'a', |from, to| from <= to + 1);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, start_at: char, end_at: char, is_step_valid: fn(u32, u32) -> bool) -> u32 {
    let input = read_lines(get_day(file!()), filename);
    let input: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let start = input.iter().position(|line| line.contains(&start_at)).unwrap();
    let start = (input[start].iter().position(|c| c == &start_at).unwrap(), start);

    let mut path = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::new();

    loop {
        let (cur, distance) = path.pop_front().unwrap();
        if !visited.insert(cur) {
            continue;
        } else if input[cur.1][cur.0] == end_at {
            return distance;
        }

        for neighbor in vec![(1, 0), (0, 1), (0, -1), (-1, 0)] {
            let neighbor = (cur.0 as i32 + neighbor.0, cur.1 as i32 + neighbor.1);
            if neighbor.0 < 0 || neighbor.0 >= input[0].len() as i32 || neighbor.1 < 0 || neighbor.1 >= input.len() as i32 {
                continue;
            }

            let neighbor = (neighbor.0 as usize, neighbor.1 as usize);
            if is_step_valid(get_height(input[cur.1][cur.0]), get_height(input[neighbor.1][neighbor.0])) {
                path.push_back((neighbor, distance + 1));
            }
        }
    }
}

fn get_height(c: char) -> u32 {
    return match c {
        'S' => 'a',
        'E' => 'z',
        _ => c,
    } as u32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let p1 = crate::solve("sample.txt", 'S', 'E', |from, to| to <= from + 1);
        let p2 = crate::solve("sample.txt", 'E', 'a', |from, to| from <= to + 1);
        assert_eq!(p1, 31);
        assert_eq!(p2, 29);
    }
}
