import os
from itertools import chain

day = os.path.dirname(os.path.realpath(__file__))


def part1(file: str):
    return unique_positions(shine(file.splitlines(), set(), (-1, 0), (1, 0)))


def part2(file: str):
    cave = file.splitlines()
    return max(
        [
            unique_positions(shine(cave, set(), pos, vel))
            for pos, vel in chain.from_iterable(
                (((x, -1), (0, 1)), ((x, len(cave)), (0, -1)), ((-1, y), (1, 0)), ((len(cave[y]), y), (-1, 0)))
                for x, y in zip(range(len(cave[0])), range(len(cave)))
            )
        ]
    )


def shine(cave: list, visited: set, old_pos: (int, int), vel: (int, int)) -> set:
    to_visit = [(old_pos, vel)]

    while len(to_visit) > 0:
        old_pos, vel = to_visit.pop()
        pos = add(old_pos, vel)

        if (pos, vel) in visited or pos[0] < 0 or pos[0] >= len(cave) or pos[1] < 0 or pos[1] >= len(cave[0]):
            continue

        visited.add((pos, vel))

        match cave[pos[1]][pos[0]]:
            case ".":
                to_visit.append((pos, vel))
            case "/":
                to_visit.append((pos, (-vel[1], -vel[0])))
            case "\\":
                to_visit.append((pos, (vel[1], vel[0])))
            case "|":
                if vel[0] == 0:
                    to_visit.append((pos, vel))
                    continue
                to_visit.append((pos, (0, -1)))
                to_visit.append((pos, (0, 1)))
            case "-":
                if vel[1] == 0:
                    to_visit.append((pos, vel))
                    continue
                to_visit.append((pos, (-1, 0)))
                to_visit.append((pos, (1, 0)))

    return visited


def add(a: (int, int), b: (int, int)) -> (int, int):
    return (a[0] + b[0], a[1] + b[1])


def unique_positions(visited: set):
    return len(set([pos for (pos, _) in visited]))


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content) == 46
        assert part2(content) == 51
