use advent_of_code::{get_day, read_lines};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Result {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_lines(get_day(file!()), filename);

    let opponent_moves = HashMap::from([("A", Move::Rock), ("B", Move::Paper), ("C", Move::Scissors)]);
    let player_moves = HashMap::from([("X", Move::Rock), ("Y", Move::Paper), ("Z", Move::Scissors)]);
    let results = HashMap::from([("X", Result::Lose), ("Y", Result::Draw), ("Z", Result::Win)]);

    let (mut p1, mut p2) = (0, 0);

    for line in input {
        let split: Vec<&str> = line.split(" ").collect();
        let opponent = *opponent_moves.get(split[0]).unwrap();

        {
            let player = *player_moves.get(split[1]).unwrap();
            let result = match (player, opponent) {
                (Move::Rock, Move::Paper) | (Move::Paper, Move::Scissors) | (Move::Scissors, Move::Rock) => Result::Lose,
                (Move::Rock, Move::Rock) | (Move::Paper, Move::Paper) | (Move::Scissors, Move::Scissors) => Result::Draw,
                (Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper) => Result::Win,
            };
            p1 += player as u32 + result as u32;
        }

        {
            let result = *results.get(split[1]).unwrap();
            let player = match (opponent, result) {
                (Move::Rock, Result::Draw) | (Move::Paper, Result::Lose) | (Move::Scissors, Result::Win) => Move::Rock,
                (Move::Rock, Result::Win) | (Move::Paper, Result::Draw) | (Move::Scissors, Result::Lose) => Move::Paper,
                (Move::Rock, Result::Lose) | (Move::Paper, Result::Win) | (Move::Scissors, Result::Draw) => Move::Scissors,
            };
            p2 += player as u32 + result as u32;
        }
    }

    return (p1, p2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 15);
        assert_eq!(p2, 12);
    }
}
