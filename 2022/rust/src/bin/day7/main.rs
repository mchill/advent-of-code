use advent_of_code::{get_day, read_file};
use std::collections::HashMap;

struct Dir {
    size: u32,
    children: Vec<String>,
}

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (u32, u32) {
    let input = read_file(get_day(file!()), filename);
    let commands = input.split("$ ");

    let (mut p1, mut p2) = (0, u32::MAX);

    let dirs: &mut HashMap<String, Dir> = &mut HashMap::new();
    let mut cur = "/".to_owned();

    for command in commands {
        let mut output = command.strip_suffix("\n").unwrap_or(command).split("\n");
        let instruction = output.next().unwrap();

        if instruction == "ls" {
            let mut dir = Dir { size: 0, children: Vec::new() };
            for line in output {
                if line.starts_with("dir") {
                    dir.children.push(cur.clone() + "/" + line.split(" ").skip(1).next().unwrap());
                } else {
                    dir.size += line.split(" ").next().unwrap().parse::<u32>().unwrap();
                }
            }
            dirs.insert(cur.clone(), dir);
        } else if instruction == "cd /" {
            cur = "/".to_owned();
        } else if instruction == "cd .." {
            cur = cur.rsplit_once("/").unwrap().0.to_owned();
        } else if instruction.starts_with("cd") {
            cur += &("/".to_owned() + instruction.split(" ").skip(1).next().unwrap());
        }
    }

    set_size("/".to_owned(), dirs);

    for dir in dirs.values() {
        if dir.size <= 100000 {
            p1 += dir.size;
        }
    }

    let min = dirs.get("/").unwrap().size - 40000000;
    for dir in dirs.values() {
        if dir.size >= min && dir.size < p2 {
            p2 = dir.size;
        }
    }

    return (p1, p2);
}

fn set_size(name: String, dirs: &mut HashMap<String, Dir>) -> u32 {
    let dir = dirs.get(&name).unwrap();
    let mut total = dir.size;

    for subdir in dir.children.clone() {
        let subsize = set_size(subdir, dirs);
        dirs.entry(name.clone()).and_modify(|dir| dir.size += subsize);
        total += subsize;
    }

    return total;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let (p1, p2) = crate::solve("sample.txt");
        assert_eq!(p1, 95437);
        assert_eq!(p2, 24933642);
    }
}
