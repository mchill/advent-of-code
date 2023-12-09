import os
from functools import reduce

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    part1 = 0
    part2 = 0

    for line in file.splitlines():
        values = [int(number) for number in line.split()]
        previous_values = [values[0]]
        next_values = [values[-1]]

        while not all(value == 0 for value in values):
            values = [values[i + 1] - values[i] for i in range(len(values) - 1)]
            previous_values.append(values[0])
            next_values.append(values[-1])

        part1 += sum(next_values)
        part2 += reduce(lambda x, y: y - x, reversed(previous_values))

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
        assert part1 == 114
        assert part2 == 2
