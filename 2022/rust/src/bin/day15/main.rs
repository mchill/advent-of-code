use advent_of_code::{get_day, read_lines};
use regex::Regex;
use std::{cmp::max, collections::HashMap};

fn main() {
    let (p1, p2) = solve("input.txt", 2000000);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, row: i64) -> (i64, i64) {
    let input = read_lines(get_day(file!()), filename);
    let mut p2 = 0;

    let mut sensors: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    let line_regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    for line in input {
        let captures = line_regex.captures(&line).unwrap();
        let sensor = (
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
        );
        let beacon = (
            captures.get(3).unwrap().as_str().parse().unwrap(),
            captures.get(4).unwrap().as_str().parse().unwrap(),
        );

        sensors.insert(sensor, beacon);
    }

    let ranges = get_unavailable_ranges(&sensors, row);
    let p1 = ranges.iter().map(|(start, end)| end - start).sum();

    for y in 0..row * 2 + 1 {
        let ranges = get_unavailable_ranges(&sensors, y);
        if ranges.len() > 1 {
            p2 = (ranges[0].1 + 1) * 4000000 + y;
            break;
        }
    }

    return (p1, p2);
}

fn get_unavailable_ranges(sensors: &HashMap<(i64, i64), (i64, i64)>, y: i64) -> Vec<(i64, i64)> {
    let mut overlapping_ranges = Vec::new();
    for (sensor, beacon) in sensors.clone() {
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let y_diff = (y - sensor.1).abs();
        if distance > y_diff {
            overlapping_ranges.push((sensor.0 - (distance - y_diff), sensor.0 + distance - y_diff));
        }
    }
    overlapping_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ranges = Vec::new();
    let (mut start, mut end) = overlapping_ranges[0];
    for range in overlapping_ranges {
        if range.0 > end + 1 {
            ranges.push((start, end));
            start = range.0;
        }
        end = max(end, range.1);
    }
    ranges.push((start, end));

    return ranges;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt", 10);
        assert_eq!(p1, 26);
        assert_eq!(p2, 56000011);
    }
}
