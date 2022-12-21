use advent_of_code::{get_day, read_lines};
use std::{collections::HashMap, panic};

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (i128, i128) {
    let input = read_lines(get_day(file!()), filename);

    let mut monkeys = HashMap::new();

    for line in input {
        let line = line.split_once(": ").unwrap();
        monkeys.insert(line.0.to_owned(), line.1.to_owned());
    }

    let p1 = calc(&monkeys, "root");

    let mut yell = monkeys.get("root").unwrap().split(" ");
    monkeys.insert("root".to_owned(), format!("{} - {}", yell.next().unwrap(), yell.skip(1).next().unwrap()));
    monkeys.insert("humn".to_owned(), "x".to_owned());
    let p2 = find_humn(&monkeys, "root", 0);

    return (p1, p2);
}

fn find_humn(monkeys: &HashMap<String, String>, monkey: &str, mut target: i128) -> i128 {
    let yell = monkeys.get(monkey).unwrap();
    if yell == "x" {
        return target;
    }

    let mut yell = yell.split(" ");
    let left = yell.next().unwrap();
    let operator = yell.next().unwrap();
    let right = yell.next().unwrap();

    panic::set_hook(Box::new(|_| {}));
    let left_result = panic::catch_unwind(|| calc(monkeys, left));
    let right_result = panic::catch_unwind(|| calc(monkeys, right));

    if left_result.is_ok() {
        let result = left_result.unwrap();
        target = match operator {
            "+" => target - result,
            "-" => result - target,
            "*" => target / result,
            "/" => result / target,
            _ => unreachable!(),
        };
        return find_humn(monkeys, right, target);
    } else if right_result.is_ok() {
        let result = right_result.unwrap();
        target = match operator {
            "+" => target - result,
            "-" => target + result,
            "*" => target / result,
            "/" => target * result,
            _ => unreachable!(),
        };
        return find_humn(monkeys, left, target);
    }
    unreachable!();
}

fn calc(monkeys: &HashMap<String, String>, monkey: &str) -> i128 {
    let yell = monkeys.get(monkey).unwrap();
    if !yell.contains(" ") {
        return yell.parse().unwrap();
    }

    let mut yell = yell.split(" ");
    let left = yell.next().unwrap();
    let operator = yell.next().unwrap();
    let right = yell.next().unwrap();

    return match operator {
        "+" => calc(monkeys, left) + calc(monkeys, right),
        "-" => calc(monkeys, left) - calc(monkeys, right),
        "*" => calc(monkeys, left) * calc(monkeys, right),
        "/" => calc(monkeys, left) / calc(monkeys, right),
        _ => unreachable!(),
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 152);
        assert_eq!(p2, 301);
    }
}
