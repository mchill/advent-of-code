use advent_of_code::{get_day, read_lines};
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = read_lines(get_day(file!()), filename);
    let lava: HashSet<[i32; 3]> = input
        .iter()
        .map(|line| line.split(",").map(|coord| coord.parse().unwrap()).collect::<Vec<i32>>().try_into().unwrap())
        .collect();

    let x_bounds = lava.iter().map(|[x, _, _]| *x).minmax().into_option().unwrap();
    let y_bounds = lava.iter().map(|[_, y, _]| *y).minmax().into_option().unwrap();
    let z_bounds = lava.iter().map(|[_, _, z]| *z).minmax().into_option().unwrap();

    let overlaps = lava.iter().fold(0, |acc, position| acc + count_overlapping_sides(&lava, *position));
    let outer_faces = flood_fill(&lava, [x_bounds, y_bounds, z_bounds], &mut HashSet::new(), [x_bounds.0, y_bounds.0, z_bounds.0]);

    return (lava.len() * 6 - overlaps, outer_faces);
}

fn count_overlapping_sides(lava: &HashSet<[i32; 3]>, position: [i32; 3]) -> usize {
    return get_neighbors(position).iter().fold(0, |acc, position| acc + lava.contains(position) as usize);
}

fn flood_fill(lava: &HashSet<[i32; 3]>, bounds: [(i32, i32); 3], checked: &mut HashSet<[i32; 3]>, position: [i32; 3]) -> usize {
    if lava.contains(&position) {
        return 1;
    } else if !checked.insert(position) || position.iter().enumerate().any(|(i, c)| *c < bounds[i].0 - 1 || *c > bounds[i].1 + 1) {
        return 0;
    }
    return get_neighbors(position)
        .iter()
        .fold(0, |acc, position| acc + flood_fill(lava, bounds, checked, *position));
}

fn get_neighbors([x, y, z]: [i32; 3]) -> Vec<[i32; 3]> {
    return vec![[x + 1, y, z], [x - 1, y, z], [x, y + 1, z], [x, y - 1, z], [x, y, z + 1], [x, y, z - 1]];
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 64);
        assert_eq!(p2, 58);
    }
}
