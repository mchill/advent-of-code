import os

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str, rate=2):
    space = file.splitlines()
    empty_rows, empty_columns = find_empty_space(space)

    galaxies = [(x, y) for y, row in enumerate(space) for x, col in enumerate(row) if col == "#"]
    distance = 0

    for galaxy1 in galaxies:
        for galaxy2 in galaxies:
            expanded_rows = [row for row in empty_rows if is_between(row, galaxy1[1], galaxy2[1])]
            expanded_columns = [col for col in empty_columns if is_between(col, galaxy1[0], galaxy2[0])]

            distance += manhattan_distance(galaxy1, galaxy2) + (len(expanded_rows) + len(expanded_columns)) * (rate - 1)

    return int(distance / 2)


def find_empty_space(space: list):
    empty_rows = [y for y, row in enumerate(space) if all(c == "." for c in row)]
    empty_columns = [x for x in range(len(space[0])) if all(c == "." for c in [row[x] for row in space])]
    return empty_rows, empty_columns


def manhattan_distance(galaxy1, galaxy2):
    return abs(galaxy1[0] - galaxy2[0]) + abs(galaxy1[1] - galaxy2[1])


def is_between(n, bound1, bound2):
    return min(bound1, bound2) < n < max(bound1, bound2)


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(main(content)))
        print("Part 2: " + str(main(content, 1000000)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert main(content) == 374
        assert main(content, 100) == 8410
