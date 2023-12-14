import os

import numpy as np

day = os.path.dirname(os.path.realpath(__file__))


def part1(file: str):
    platform = get_platform(file)
    platform = np.rot90(platform)

    tilt_west(platform)

    platform = np.rot90(platform, 3)
    return get_north_load(platform)


def part2(file: str):
    platform = get_platform(file)
    platform = np.rot90(platform)

    cache = {}
    remaining_cycles = None

    for i in range(1000000000):
        key = "".join(["".join(row) for row in platform])
        if key in cache:
            remaining_cycles = (1000000000 - i) % (i - cache[key])
            break
        cache[key] = i

        for _ in range(4):
            tilt_west(platform)
            platform = np.rot90(platform, 3)

    for _ in range(remaining_cycles):
        for _ in range(4):
            tilt_west(platform)
            platform = np.rot90(platform, 3)

    platform = np.rot90(platform, 3)
    return get_north_load(platform)


def get_platform(file: str):
    platform = ["#" + line + "#" for line in file.splitlines()]
    platform.insert(0, "#" * len(platform[0]))
    platform.append("#" * len(platform[0]))
    return np.array([list(row) for row in platform])


def tilt_west(platform: np.ndarray):
    for y in range(len(platform)):
        line = platform[y]
        new_line = []
        last = 0

        for i in [i for i, c in enumerate(line) if c == "#"]:
            section = list(line[last:i])
            new_line.extend("O" * section.count("O"))
            new_line.extend("." * (len(section) - section.count("O")))
            new_line.append("#")
            last = i + 1

        platform[y] = new_line


def get_north_load(platform):
    return sum([sum([i for i, c in enumerate(line) if c == "O"]) for line in np.rot90(platform, 3)])


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content) == 136
        assert part2(content) == 64
