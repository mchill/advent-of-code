use advent_of_code::{get_day, read_file};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
    divisor: u64,
}

fn main() {
    let p1 = crate::solve("input.txt", 20, true);
    let p2 = crate::solve("input.txt", 10000, false);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str, rounds: usize, worry: bool) -> u64 {
    let input = read_file(get_day(file!()), filename);
    let input = input
        .strip_suffix("\n")
        .unwrap()
        .split("\n\n")
        .map(|line| line.split("\n").collect::<Vec<&str>>());

    let mut monkeys = Vec::new();
    for chunk in input {
        let items = chunk[1]
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let mut formula = chunk[2].split("= ").skip(1).next().unwrap().split_whitespace().skip(1);
        let (operator, operand) = (formula.next().unwrap(), formula.next().unwrap().parse::<u64>());
        let operation: Box<dyn Fn(u64) -> u64> = match operator {
            "+" => Box::new(move |item| item + operand.clone().unwrap_or(item)),
            "*" => Box::new(move |item| item * operand.clone().unwrap_or(item)),
            _ => unreachable!(),
        };

        let divisor = chunk[3].rsplit_once(" ").unwrap().1.parse().unwrap();
        let if_true = chunk[4].rsplit_once(" ").unwrap().1.parse().unwrap();
        let if_false = chunk[5].rsplit_once(" ").unwrap().1.parse().unwrap();
        let test = Box::new(move |item| if item % divisor == 0 { if_true } else { if_false });

        monkeys.push(Monkey {
            items,
            operation,
            test,
            divisor,
        });
    }

    let mut passes: HashMap<usize, u64> = (0..monkeys.len()).map(|i| (i, 0)).collect();
    let lcm = monkeys.iter().fold(1, |acc, monkey| acc * monkey.divisor);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                *passes.get_mut(&i).unwrap() += 1;
                let monkey = &mut monkeys[i];
                let mut item = (monkey.operation)(monkey.items.pop_front().unwrap()) % lcm;
                if worry {
                    item = item / 3;
                }
                let pass_to = (monkey.test)(item);
                monkeys[pass_to].items.push_back(item);
            }
        }
    }

    let mut passes = passes.values().sorted().rev();
    return *passes.next().unwrap() * *passes.next().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let p1 = crate::solve("sample.txt", 20, true);
        let p2 = crate::solve("sample.txt", 10000, false);
        assert_eq!(p1, 10605);
        assert_eq!(p2, 2713310158);
    }
}
