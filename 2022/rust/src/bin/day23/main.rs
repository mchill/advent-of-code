use advent_of_code::{get_day, read_lines};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);
    let mut p1 = 0;

    let mut map = HashSet::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i32, y as i32));
            }
        }
    }

    let mut round = 0;
    loop {
        let mut moves = HashMap::new();
        let mut conflicts = HashSet::new();

        for (x, y) in map.iter() {
            let is_north_empty = !map.contains(&(*x - 1, *y - 1)) && !map.contains(&(*x, *y - 1)) && !map.contains(&(*x + 1, *y - 1));
            let is_south_empty = !map.contains(&(*x - 1, *y + 1)) && !map.contains(&(*x, *y + 1)) && !map.contains(&(*x + 1, *y + 1));
            let is_west_empty = !map.contains(&(*x - 1, *y - 1)) && !map.contains(&(*x - 1, *y)) && !map.contains(&(*x - 1, *y + 1));
            let is_east_empty = !map.contains(&(*x + 1, *y - 1)) && !map.contains(&(*x + 1, *y)) && !map.contains(&(*x + 1, *y + 1));

            if is_north_empty && is_south_empty && is_west_empty && is_east_empty {
                continue;
            }

            for try_move in 0..4 {
                let (move_to, should_move) = match (round + try_move) % 4 {
                    0 => ((*x, *y - 1), is_north_empty),
                    1 => ((*x, *y + 1), is_south_empty),
                    2 => ((*x - 1, *y), is_west_empty),
                    3 => ((*x + 1, *y), is_east_empty),
                    _ => unreachable!(),
                };

                if should_move {
                    if moves.contains_key(&move_to) {
                        conflicts.insert(move_to);
                    }
                    moves.insert(move_to, (*x, *y));
                    break;
                }
            }
        }

        for conflict in conflicts {
            moves.remove(&conflict);
        }

        for (move_to, move_from) in moves.iter() {
            map.remove(move_from);
            map.insert(move_to.clone());
        }

        round += 1;

        if round == 10 {
            let (min_x, max_x) = map.iter().map(|(x, _)| *x).minmax_by(|a, b| a.cmp(&b)).into_option().unwrap();
            let (min_y, max_y) = map.iter().map(|(_, y)| *y).minmax_by(|a, b| a.cmp(&b)).into_option().unwrap();
            p1 = (min_x.abs_diff(max_x) + 1) * (min_y.abs_diff(max_y) + 1) - map.len() as u32;
        }

        if moves.is_empty() {
            return (p1, round);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 110);
        assert_eq!(p2, 20);
    }
}
