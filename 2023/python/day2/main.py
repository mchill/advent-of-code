import operator
import os
from functools import reduce

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    part1 = 0
    part2 = 0

    for line in file.splitlines():
        maxes = {"blue": 0, "green": 0, "red": 0}
        left, right = line.split(": ")

        for amount, color in [handful.split(" ") for handful in right.replace(";", ",").split(", ")]:
            maxes[color] = max(maxes[color], int(amount))

        part2 += reduce(operator.mul, maxes.values())
        if maxes["blue"] <= 14 and maxes["green"] <= 13 and maxes["red"] <= 12:
            part1 += int(left.split(" ")[1])

    return part1, part2


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        part1, part2 = main(content)
        print("Part 1: " + str(part1))
        print("Part 2: " + str(part2))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        part1, part2 = main(content)
        assert part1 == 8
        assert part2 == 2286
