import os
import sys
from functools import cache

sys.setrecursionlimit(10000)

day = os.path.dirname(os.path.realpath(__file__))


garden = None


def part1(file: str, steps: int):
    global garden

    garden = file.replace("S", ".").splitlines()
    start = (len(garden) // 2, len(garden) // 2)

    return len(step(start, steps))


@cache
def step(position, remaining_steps):
    global garden

    if remaining_steps == 0:
        return {position}

    positions = set()

    for offset in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
        new_position = (position[0] + offset[0], position[1] + offset[1])
        if garden[new_position[1]][new_position[0]] == ".":
            positions.update(step(new_position, remaining_steps - 1))

    return positions


def part2(file: str, steps: int):
    garden = [list(row) for row in file.replace("S", ".").splitlines()]

    d = len(garden)
    r = d // 2
    n = (steps - r + 1) // d

    start = (r, r)
    flood_fill(garden, start)

    blue_square = [
        ["0" if c == " " and distance(x, y, r) % 2 == 1 else "." for x, c in enumerate(garden[y])] for y in range(d)
    ]
    green_square = [
        ["0" if c == " " and distance(x, y, r) % 2 == 0 else "." for x, c in enumerate(garden[y])] for y in range(d)
    ]
    blue_diamond = [
        ["0" if c == "0" and in_diamond(x, y, r) else "." for x, c in enumerate(blue_square[y])] for y in range(d)
    ]
    green_diamond = [
        ["0" if c == "0" and in_diamond(x, y, r) else "." for x, c in enumerate(green_square[y])] for y in range(d)
    ]

    blue_square_steps = sum([row.count("0") for row in blue_square])
    green_square_steps = sum([row.count("0") for row in green_square])
    blue_diamond_steps = sum([row.count("0") for row in blue_diamond])
    green_diamond_steps = sum([row.count("0") for row in green_diamond])

    return (
        (n**2 + n) * blue_square_steps
        + (n + 1) * blue_diamond_steps
        + (n**2 + n) * green_square_steps
        - (n) * green_diamond_steps
    )


def flood_fill(garden, start):
    x, y = start
    if x < 0 or x >= len(garden) or y < 0 or y >= len(garden[0]) or garden[y][x] != ".":
        return
    garden[y][x] = " "
    flood_fill(garden, (x + 1, y))
    flood_fill(garden, (x - 1, y))
    flood_fill(garden, (x, y + 1))
    flood_fill(garden, (x, y - 1))


def distance(x, y, radius):
    return abs(x - radius) + abs(y - radius)


def in_diamond(x, y, radius):
    return distance(x, y, radius) <= radius


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content, 64)))
        print("Part 2: " + str(part2(content, 26501365)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content, 6) == 16
