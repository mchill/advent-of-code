use advent_of_code::{get_day, read_lines};

fn main() {
    let p1 = solve("input.txt", 1, 1);
    let p2 = solve("input.txt", 811589153, 10);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, multiplier: i64, repeat: u32) -> i64 {
    let input = read_lines(get_day(file!()), filename);

    let mut sequence: Vec<(usize, i64)> = input
        .iter()
        .enumerate()
        .map(|(i, line)| (i, line.parse::<i64>().unwrap() * multiplier))
        .collect();

    for _ in 0..repeat {
        for i in 0..input.len() {
            let index = sequence.iter().position(|(index, _)| *index == i).unwrap();
            let element = sequence.remove(index);
            let new_index = (index as i64 + element.1).rem_euclid(input.len() as i64 - 1) as usize;
            sequence.insert(new_index, element);
        }
    }

    let index = sequence.iter().position(|(_, value)| *value == 0).unwrap();
    return sequence[(index + 1000) % input.len()].1 + sequence[(index + 2000) % input.len()].1 + sequence[(index + 3000) % input.len()].1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let p1 = crate::solve("sample.txt", 1, 1);
        let p2 = crate::solve("sample.txt", 811589153, 10);
        assert_eq!(p1, 3);
        assert_eq!(p2, 1623178306);
    }
}
