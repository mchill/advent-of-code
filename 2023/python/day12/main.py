import os
import re

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    part1 = 0
    part2 = 0

    for row in file.splitlines():
        springs = row.split(" ")[0]
        errors = [int(error) for error in row.split(" ")[1].split(",")]
        part1 += get_possibilities(springs, 0, errors, {})
        part2 += get_possibilities("?".join([springs] * 5), 0, errors * 5, {})

    return part1, part2


def get_possibilities(springs: str, index: int, errors: list, cache: dict) -> int:
    error_groups = [len(group) for group in re.findall(r"(#+)", springs[:index])]
    if index == len(springs):
        return 1 if error_groups == errors else 0

    if springs[index] == "#":
        return get_possibilities(springs, index + 1, errors, cache)

    if springs[index] == "?":
        s1 = springs[:index] + "." + springs[index + 1 :]
        s2 = springs[:index] + "#" + springs[index + 1 :]
        return get_possibilities(s1, index, errors, cache) + get_possibilities(s2, index, errors, cache)

    if errors[: len(error_groups)] != error_groups:
        return 0

    key = (index, tuple(error_groups))
    if key not in cache:
        cache[key] = get_possibilities(springs, index + 1, errors, cache)
    return cache[key]


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
        assert part1 == 21
        assert part2 == 525152
