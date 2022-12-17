use advent_of_code::{get_day, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::max,
    collections::{BTreeSet, HashSet},
    collections::{HashMap, VecDeque},
};

struct Valve {
    rate: u32,
    tunnels: Vec<(String, u32)>,
}

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);

    let mut valves: HashMap<String, Valve> = HashMap::new();
    let line_regex = Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    for line in input {
        let captures = line_regex.captures(&line).unwrap();
        let valve = captures.get(1).unwrap().as_str().to_owned();
        let rate = captures.get(2).unwrap().as_str().parse().unwrap();
        let tunnels = captures.get(3).unwrap().as_str().split(", ").map(|valve| (valve.to_owned(), 1)).collect();
        valves.insert(valve, Valve { rate, tunnels });
    }

    simplify_graph(&mut valves);

    let p1 = tick(&valves, BTreeSet::new(), &mut HashMap::new(), "AA", 30, false);
    let p2 = tick(&valves, BTreeSet::new(), &mut HashMap::new(), "AA", 26, true);

    return (p1, p2);
}

fn simplify_graph(valves: &mut HashMap<String, Valve>) {
    *valves = valves
        .iter()
        .map(|(name, valve)| {
            let tunnels = valves
                .iter()
                .filter(|(dest_name, dest_valve)| name != *dest_name && dest_valve.rate > 0)
                .map(|(dest, _)| (dest.clone(), get_distance(&valves, name.clone(), dest.clone())))
                .collect();
            return (name.clone(), Valve { rate: valve.rate, tunnels });
        })
        .filter(|(name, valve)| name == "AA" || valve.rate > 0)
        .collect();
}

fn get_distance(valves: &HashMap<String, Valve>, source: String, target: String) -> u32 {
    let mut open = VecDeque::from([(source, 1)]);
    let mut closed = HashSet::new();
    loop {
        let (cur, distance) = open.pop_front().unwrap();
        if cur == target {
            return distance;
        }
        if closed.insert(cur.clone()) {
            for (tunnel, _) in &valves.get(&cur).unwrap().tunnels {
                open.push_back((tunnel.clone(), distance + 1));
            }
        }
    }
}

fn tick(valves: &HashMap<String, Valve>, mut open: BTreeSet<String>, state: &mut HashMap<String, u32>, cur: &str, remaining: u32, first: bool) -> u32 {
    open.insert(cur.to_owned());
    let key = format!("{};{};{};{}", open.iter().join(","), cur, remaining.to_string(), first.to_string());
    if state.contains_key(&key) {
        return *state.get(&key).unwrap();
    }

    let mut released = if first { tick(valves, open.clone(), state, "AA", 26, false) } else { 0 };
    for (tunnel, cost) in &valves[cur].tunnels {
        if !open.contains(tunnel) && remaining > *cost {
            released = max(released, tick(valves, open.clone(), state, tunnel, remaining - *cost, first));
        }
    }

    released = valves.get(cur).unwrap().rate * remaining + released;
    state.insert(key, released);
    return released;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 1651);
        assert_eq!(p2, 1707);
    }
}
