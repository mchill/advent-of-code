import os

from ranges import Range, RangeSet

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    sections = file.split("\n\n")
    seeds = [int(seed) for seed in sections[0].split(": ")[1].split()]
    maps = []

    for section in sections[1:]:
        lines = [[int(number) for number in line.split()] for line in section.splitlines()[1:]]
        maps.append(lines)

    part1 = min([get_lowest([seed, seed + 1], maps, 0) for seed in seeds])
    part2 = min([get_lowest([seeds[i], seeds[i] + seeds[i + 1]], maps, 0) for i in range(0, len(seeds), 2)])

    return part1, part2


def get_lowest(my_range, maps, map_index):
    if map_index == len(maps):
        return my_range[0]

    lowest = float("inf")

    for [destination_start, source_start, length] in maps[map_index]:
        source_range = [source_start, source_start + length]
        overlap_range = [max(source_range[0], my_range[0]), min(source_range[1], my_range[1])]
        if overlap_range[0] < overlap_range[1]:
            destination_range = [
                overlap_range[0] - source_start + destination_start,
                overlap_range[1] - source_start + destination_start,
            ]
            lowest = min(lowest, get_lowest(destination_range, maps, map_index + 1))

    r1 = RangeSet([Range(source_start, source_start + length) for [_, source_start, length] in maps[map_index]])
    r2 = RangeSet([Range(my_range[0], my_range[1])])
    for range in r2 - r1:
        lowest = min(lowest, get_lowest([range.start, range.end], maps, map_index + 1))

    return lowest


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
        assert part1 == 35
        assert part2 == 46
