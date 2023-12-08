import itertools
import math
import os

day = os.path.dirname(os.path.realpath(__file__))


def part1(file: str):
    directions, network = parse_file(file)
    return get_steps(directions, network, "AAA", lambda c: c == "ZZZ")


def part2(file: str):
    directions, network = parse_file(file)
    starts = [key for key in network.keys() if key.endswith("A")]
    return math.lcm(*[get_steps(directions, network, start, lambda c: c.endswith("Z")) for start in starts])


def parse_file(file: str):
    directions, nodes = file.split("\n\n")

    network = {}
    for node in nodes.splitlines():
        start, end = node.split(" = ")
        left, right = end[1:-1].split(", ")
        network[start] = (left, right)

    return directions, network


def get_steps(directions: str, network: dict, current: str, condition):
    steps = 0
    for dir in itertools.cycle(directions):
        current = network[current][0] if dir == "L" else network[current][1]
        steps += 1
        if condition(current):
            return steps


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample1.txt") as file:
        content = file.read()
        assert part1(content) == 2

    with open(day + "/sample2.txt") as file:
        content = file.read()
        assert part2(content) == 6
