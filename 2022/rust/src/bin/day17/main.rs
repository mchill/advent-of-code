use advent_of_code::{get_day, read_file};
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

struct Shape {
    width: i64,
    height: i64,
    blocks: Vec<(i64, i64)>,
}

impl Shape {
    fn new(blocks: Vec<(i64, i64)>) -> Shape {
        let width = blocks.iter().map(|block| block.0).max().unwrap() + 1;
        let height = blocks.iter().map(|block| block.1).max().unwrap() + 1;
        return Shape { width, height, blocks };
    }

    fn collides(&self, chamber: &HashSet<(i64, i64)>, (x, y): (i64, i64)) -> bool {
        return x < 0 || x > 7 - self.width || self.blocks.iter().any(|block| chamber.contains(&(x + block.0, y + block.1)));
    }

    fn insert(&self, chamber: &mut HashSet<(i64, i64)>, (x, y): (i64, i64)) {
        for block in self.blocks.iter() {
            chamber.insert((x + block.0, y + block.1));
        }
    }
}

fn main() {
    let p1 = solve("input.txt", 2022);
    let p2 = solve("input.txt", 1000000000000);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, count: usize) -> i64 {
    let input = read_file(get_day(file!()), filename);
    let input = input.strip_suffix("\n").unwrap();
    let total_chars = input.len() as u64;
    let mut input = input.chars().cycle();

    let shapes: Vec<Shape> = vec![
        Shape::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        Shape::new(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Shape::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Shape::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        Shape::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
    ];

    let mut chamber: HashSet<(i64, i64)> = (0..7).into_iter().map(|x| (x, 0)).collect();
    let mut height = 0;
    let mut cache: HashMap<String, (usize, i64)> = HashMap::new();
    let mut offset = 0;
    let mut char_index: u64 = 0;
    let mut shape_index = 0;

    while shape_index < count {
        let key = format!(
            "{};{};{}",
            shape_index % shapes.len(),
            char_index % total_chars,
            get_chamber_hash(&chamber, height)
        );
        if cache.contains_key(&key) {
            let (cached_shape_index, cached_height) = cache.get(&key).unwrap();
            let cycle_size = shape_index - cached_shape_index;
            let height_diff = height - cached_height;
            let remaining = count - shape_index;
            let remaining_cycles = remaining / cycle_size;
            shape_index += cycle_size * remaining_cycles;
            offset += height_diff * remaining_cycles as i64;
        } else {
            cache.insert(key, (shape_index, height));
        }

        let shape = &shapes[shape_index % shapes.len()];
        let mut position = (2, height + 4);

        loop {
            char_index += 1;
            let new_position = match input.next().unwrap() {
                '<' => (position.0 - 1, position.1),
                '>' => (position.0 + 1, position.1),
                _ => unreachable!(),
            };
            if !shape.collides(&chamber, new_position) {
                position = new_position;
            }

            let new_position = (position.0, position.1 - 1);
            if !shape.collides(&chamber, new_position) {
                position = new_position;
                continue;
            }

            shape.insert(&mut chamber, position);
            height = max(height, new_position.1 + shape.height);
            break;
        }

        shape_index += 1;
    }

    return height + offset;
}

fn get_chamber_hash(chamber: &HashSet<(i64, i64)>, height: i64) -> u8 {
    let mut hash = Vec::new();
    for x in 0..7 {
        let mut y = height;
        while y > 0 && !chamber.contains(&(x, y)) {
            y -= 1;
        }
        hash.push(y as u8);
    }
    return hash.iter().fold(0, |result, &bit| (result << 1) ^ bit);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let p1 = crate::solve("sample.txt", 2022);
        let p2 = crate::solve("sample.txt", 1000000000000);

        assert_eq!(p1, 3068);
        assert_eq!(p2, 1514285714288);
    }
}
