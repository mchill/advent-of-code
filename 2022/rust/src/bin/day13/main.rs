use advent_of_code::{get_day, read_file};
use serde_json::{json, Value};
use std::cmp::Ordering;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = read_file(get_day(file!()), filename);
    let mut input: Vec<Value> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    let p1 = input
        .chunks(2)
        .enumerate()
        .filter(|(_, pair)| compare(&pair[0], &pair[1]) == Ordering::Less)
        .fold(0, |acc, (i, _)| acc + i + 1);

    input.push(json![[[2]]]);
    input.push(json![[[6]]]);
    input.sort_by(|left, right| compare(left, right));
    let p2 = (input.iter().position(|item| *item == json![[[2]]]).unwrap() + 1) * (input.iter().position(|item| *item == json![[[6]]]).unwrap() + 1);

    return (p1, p2);
}

fn compare(left: &Value, right: &Value) -> Ordering {
    if left.is_number() && right.is_number() {
        return left.as_i64().unwrap().cmp(&right.as_i64().unwrap());
    }

    let left = if left.is_number() { json![[left]] } else { left.clone() };
    let right = if right.is_number() { json![[right]] } else { right.clone() };

    let left = left.as_array().unwrap();
    let right = right.as_array().unwrap();

    for (left_item, right_item) in left.iter().zip(right.iter()) {
        let comparison = compare(left_item, right_item);
        if comparison != Ordering::Equal {
            return comparison;
        }
    }

    return left.len().cmp(&right.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 13);
        assert_eq!(p2, 140);
    }
}
