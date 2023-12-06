import operator
import os
from functools import reduce

day = os.path.dirname(os.path.realpath(__file__))


def part1(file: str):
    return race(*[[int(number) for number in line.split(":")[1].split()] for line in file.splitlines()])


def part2(file: str):
    return race(*[[int(line.split(":")[1].replace(" ", ""))] for line in file.splitlines()])


def race(times, records):
    return reduce(operator.mul, [sum(1 for s in range(1, t) if s * (t - s) > r) for t, r in zip(times, records)])


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content) == 288
        assert part2(content) == 71503
