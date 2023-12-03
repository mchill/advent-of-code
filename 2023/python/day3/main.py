import os
from collections import defaultdict

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    part1 = 0
    part2 = 0

    lines = ["." + line + "." for line in file.splitlines()]
    lines.insert(0, "." * len(lines[0]))
    lines.append("." * len(lines[0]))

    start_x = None
    gears = defaultdict(list)

    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            if c.isdigit() and start_x is None:
                start_x = x
                continue

            if c.isdigit() or start_x is None:
                continue

            number = int(lines[y][start_x:x])
            symbols = get_adjacent_symbols(lines, [start_x, x - 1], [y, y])

            if len(symbols) > 0:
                part1 += number

            for symbol in symbols:
                if lines[symbol[0]][symbol[1]] == "*":
                    gears[str(symbol[0]) + "," + str(symbol[1])].append(number)

            start_x = None

    for numbers in gears.values():
        if len(numbers) == 2:
            part2 += numbers[0] * numbers[1]

    return part1, part2


def get_adjacent_symbols(lines, xrange, yrange):
    symbols = []
    for y in range(yrange[0] - 1, yrange[1] + 2):
        for x in range(xrange[0] - 1, xrange[1] + 2):
            if not lines[y][x].isdigit() and lines[y][x] != ".":
                symbols.append([y, x])
    return symbols


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
        assert part1 == 4361
        assert part2 == 467835
