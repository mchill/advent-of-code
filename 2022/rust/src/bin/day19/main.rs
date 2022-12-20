use advent_of_code::{add_arrays, get_day, read_lines, subtract_arrays};
use regex::Regex;
use std::{cmp::max, collections::HashMap};

struct Blueprint {
    robots: [[u32; 4]; 4],
    maxes: [u32; 4],
}

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);
    let (mut p1, mut p2) = (0, 1);

    let mut blueprints = Vec::new();
    let line_regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    for line in input {
        let captures: Vec<u32> = line_regex
            .captures(&line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|n| n.unwrap().as_str().parse().unwrap())
            .collect();
        let robots = [
            [captures[0], 0, 0, 0],
            [captures[1], 0, 0, 0],
            [captures[2], captures[3], 0, 0],
            [captures[4], 0, captures[5], 0],
        ];
        let maxes: [u32; 4] = (0..4)
            .map(|i| robots.iter().map(|robot| robot[i]).max().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();
        blueprints.push(Blueprint { robots, maxes })
    }

    for (i, blueprint) in blueprints.iter().enumerate() {
        p1 += (i as u32 + 1) * tick(blueprint, &mut HashMap::new(), [1, 0, 0, 0], [0, 0, 0, 0], 24);
    }

    for blueprint in blueprints.iter().take(3) {
        p2 *= tick(blueprint, &mut HashMap::new(), [1, 0, 0, 0], [0, 0, 0, 0], 32);
    }

    return (p1, p2);
}

fn tick(blueprint: &Blueprint, checked: &mut HashMap<[u32; 8], u32>, robots: [u32; 4], resources: [u32; 4], remaining: u32) -> u32 {
    if remaining == 0 {
        return resources[3];
    }

    let key = [robots, resources].concat().try_into().unwrap();
    if checked.contains_key(&key) && remaining <= *checked.get(&key).unwrap() {
        return 0;
    }
    checked.insert(key, remaining);

    let mut max_geodes = try_create_robot(blueprint, checked, robots, resources, remaining, 3);
    if max_geodes > 0 {
        return max_geodes;
    }

    let paths: Vec<u32> = (0..3).map(|i| try_create_robot(blueprint, checked, robots, resources, remaining, i)).collect();
    max_geodes = *paths.iter().max().unwrap();
    if !paths.contains(&0) {
        return max_geodes;
    }

    return max(max_geodes, tick(blueprint, checked, robots, add_arrays(resources, robots), remaining - 1));
}

fn try_create_robot(blueprint: &Blueprint, checked: &mut HashMap<[u32; 8], u32>, robots: [u32; 4], resources: [u32; 4], remaining: u32, i: usize) -> u32 {
    if (blueprint.maxes[i] != 0 && robots[i] >= blueprint.maxes[i]) || !resources.into_iter().zip(blueprint.robots[i]).all(|(resource, cost)| resource >= cost)
    {
        return 0;
    }
    let mut new_robots = robots.clone();
    new_robots[i] += 1;
    let new_resources = subtract_arrays(add_arrays(resources, robots), blueprint.robots[i]);
    return tick(blueprint, checked, new_robots, new_resources, remaining - 1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 33);
        assert_eq!(p2, 3472);
    }
}
