pub mod shapes;

use crate::Side::*;
use advent_of_code::{get_day, read_file};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Side {
    Right = 0,
    Bottom = 1,
    Left = 2,
    Top = 3,
}

pub struct Face {
    map: Vec<Vec<char>>,
    offset: (usize, usize),
    sides: HashMap<Side, Edge>,
}

pub struct Edge {
    face: char,
    edge: Side,
}

fn main() {
    let p1 = solve("input.txt", &mut shapes::get_input_p1(), 50);
    let p2 = solve("input.txt", &mut shapes::get_input_p2(), 50);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, faces: &mut HashMap<char, Face>, size: usize) -> usize {
    let input = read_file(get_day(file!()), filename);
    let (map, path) = input.strip_suffix("\n").unwrap().split_once("\n\n").unwrap();
    let map: Vec<Vec<char>> = map.split("\n").map(|line| line.chars().collect()).collect();
    let path = path.replace("L", " L ").replace("R", " R ");

    for (_, face) in faces.iter_mut() {
        for y in face.offset.1..face.offset.1 + size {
            face.map.push(map[y][face.offset.0..face.offset.0 + size].to_vec());
        }
    }

    let mut face = faces.get(&'A').unwrap();
    let mut pos = (face.map[0].iter().position(|c| *c == '.').unwrap(), 0);
    let mut dir = Right;

    for instruction in path.split(" ") {
        if instruction == "L" || instruction == "R" {
            dir = rotate(dir, instruction == "R", 1);
            continue;
        }

        for _ in 0..instruction.parse::<usize>().unwrap() {
            let mut new_face = face;
            let mut new_dir = dir;
            let (mut new_pos, overflowed) = forward(pos, dir, size);

            if overflowed {
                let side = face.sides.get(&dir).unwrap();
                new_face = faces.get(&side.face).unwrap();
                new_dir = rotate(side.edge, true, 2);
                new_pos = traverse_edge(pos, size, dir, side.edge);
            }

            if new_face.map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                break;
            }

            face = new_face;
            pos = (new_pos.0 as usize, new_pos.1 as usize);
            dir = new_dir;
        }
    }

    return 1000 * (pos.1 + face.offset.1 + 1) + 4 * (pos.0 + face.offset.0 + 1) + dir as usize;
}

fn forward((x, y): (usize, usize), dir: Side, size: usize) -> ((usize, usize), bool) {
    return match dir {
        Right => ((x + 1, y), x == size - 1),
        Bottom => ((x, y + 1), y == size - 1),
        Left => ((x.max(1) - 1, y), x == 0),
        Top => ((x, y.max(1) - 1), y == 0),
    };
}

fn rotate(dir: Side, clockwise: bool, n: usize) -> Side {
    return [Right, Bottom, Left, Top][(dir as usize + 4 + n * clockwise as usize - n * !clockwise as usize).rem_euclid(4)];
}

fn traverse_edge((x, y): (usize, usize), size: usize, from: Side, to: Side) -> (usize, usize) {
    return match (from, to) {
        (Right, Right) | (Left, Left) | (Bottom, Top) | (Top, Bottom) => (x, size - 1 - y),
        (Right, Bottom) | (Bottom, Right) | (Left, Top) | (Top, Left) => (y, x),
        (Right, Top) | (Top, Right) | (Left, Bottom) | (Bottom, Left) => (size - 1 - y, size - 1 - x),
        (Bottom, Bottom) | (Top, Top) | (Right, Left) | (Left, Right) => (size - 1 - x, y),
    };
}

#[cfg(test)]
mod tests {
    use crate::shapes;

    #[test]
    fn test() {
        let p1 = crate::solve("sample.txt", &mut shapes::get_sample_p1(), 4);
        let p2 = crate::solve("sample.txt", &mut shapes::get_sample_p2(), 4);
        assert_eq!(p1, 6032);
        assert_eq!(p2, 5031);
    }
}
