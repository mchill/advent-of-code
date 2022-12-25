use advent_of_code::{add_arrays, get_day, read_lines};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Eq)]
struct State {
    f: u32,
    time: u32,
    position: [i32; 2],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Blizzard {
    position: [i32; 2],
    direction: [i32; 2],
}

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);

    let mut blizzards = Vec::new();
    let width = input[0].len() as i32 - 2;
    let height = input.len() as i32 - 2;

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' && c != '#' {
                blizzards.push(Blizzard {
                    position: [x as i32 - 1, y as i32 - 1],
                    direction: match c {
                        '>' => [1, 0],
                        'v' => [0, 1],
                        '<' => [-1, 0],
                        '^' => [0, -1],
                        _ => unreachable!(),
                    },
                });
            }
        }
    }

    let result = traverse(&blizzards, width, height, [0, -1], [width - 1, height], 0, 3);

    return (*result.first().unwrap(), *result.last().unwrap());
}

fn traverse(blizzards: &Vec<Blizzard>, width: i32, height: i32, start: [i32; 2], end: [i32; 2], time: u32, remaining: usize) -> Vec<u32> {
    if remaining == 0 {
        return Vec::new();
    }

    let mut open = BinaryHeap::from([State { f: 0, position: start, time }]);
    let mut closed = HashSet::new();

    while let Some(State { f: _, time, position }) = open.pop() {
        if !closed.insert((time, position)) {
            continue;
        }

        let mut valley: HashSet<[i32; 2]> = HashSet::new();
        for blizzard in blizzards {
            if (position[0] >= blizzard.position[0] - 1 && position[0] <= blizzard.position[0] + 1)
                || (position[1] >= blizzard.position[1] - 1 && position[1] <= blizzard.position[1] + 1)
            {
                let blizzard_pos = [
                    (blizzard.position[0] + blizzard.direction[0] * (time + 1) as i32).rem_euclid(width),
                    (blizzard.position[1] + blizzard.direction[1] * (time + 1) as i32).rem_euclid(height),
                ];
                valley.insert(blizzard_pos);
            }
        }

        for option in [[1, 0], [0, 1], [-1, 0], [0, -1], [0, 0]] {
            let new_pos = add_arrays(position, option);

            if new_pos == end {
                let mut result = traverse(blizzards, width, height, end, start, time + 1, remaining - 1);
                result.insert(0, time + 1);
                return result;
            }

            if new_pos == start || new_pos[0] >= 0 && new_pos[0] <= width - 1 && new_pos[1] >= 0 && new_pos[1] <= height - 1 && !valley.contains(&new_pos) {
                open.push(State {
                    f: time + new_pos[0].abs_diff(end[0]) + new_pos[1].abs_diff(end[1]),
                    position: new_pos,
                    time: time + 1,
                });
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 18);
        assert_eq!(p2, 54);
    }
}
