import os
import re
from collections import defaultdict
from functools import reduce

day = os.path.dirname(os.path.realpath(__file__))


def part1(file: str):
    return sum([hash(step) for step in file.strip().split(",")])


def part2(file: str):
    boxes = defaultdict(list)
    lengths = {}

    for step in file.strip().split(","):
        (label, operation, focal_length) = re.match(r"([a-z]+)(=|-)([0-9])?", step).groups()
        box = hash(label)

        if operation == "-":
            if label in boxes[box]:
                boxes[box].remove(label)
            continue

        lengths[label] = int(focal_length)
        if label not in boxes[box]:
            boxes[box].append(label)

    return sum([(box + 1) * (i + 1) * lengths[lens] for box, lenses in boxes.items() for i, lens in enumerate(lenses)])


def hash(label: str):
    return reduce(lambda value, c: (value + ord(c)) * 17 % 256, label, 0)


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content) == 1320
        assert part2(content) == 145
