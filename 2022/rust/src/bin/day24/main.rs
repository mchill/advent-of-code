use advent_of_code::{add_arrays, get_day, read_lines};
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Blizzard {
    position: [i32; 2],
    direction: [i32; 2],
}

impl Blizzard {
    fn forward(&mut self, width: i32, height: i32) {
        let new_pos = add_arrays(self.position, self.direction);
        self.position = [new_pos[0].rem_euclid(width), new_pos[1].rem_euclid(height)];
    }
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

    let p1 = traverse(blizzards.clone(), width, height, [0, -1], [width - 1, height], 1);
    let p2 = traverse(blizzards, width, height, [0, -1], [width - 1, height], 3);

    return (p1, p2);
}

fn traverse(initial_blizzards: Vec<Blizzard>, width: i32, height: i32, start: [i32; 2], end: [i32; 2], remaining: usize) -> u32 {
    if remaining == 0 {
        return 0;
    }

    let mut open = VecDeque::from([(initial_blizzards, start, 0)]);
    let mut closed = HashSet::new();

    loop {
        let (mut blizzards, pos, time) = open.pop_front().unwrap();
        if pos == end {
            return time + traverse(blizzards, width, height, end, start, remaining - 1);
        }
        if !closed.insert((time, pos)) {
            continue;
        }

        let mut valley: HashSet<[i32; 2]> = HashSet::new();
        for blizzard in &mut blizzards {
            blizzard.forward(width, height);
            valley.insert(blizzard.position);
        }

        for option in [[1, 0], [0, 1], [-1, 0], [0, -1], [0, 0]] {
            let new_pos = add_arrays(pos, option);

            if (pos == start && new_pos == start)
                || new_pos == end
                || new_pos[0] >= 0 && new_pos[0] <= width - 1 && new_pos[1] >= 0 && new_pos[1] <= height - 1 && !valley.contains(&new_pos)
            {
                open.push_back((blizzards.clone(), new_pos, time + 1));
            }
        }
    }
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
