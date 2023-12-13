import os

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    part1 = 0
    part2 = 0

    grids = file.split("\n\n")
    for grid in grids:
        summaries = get_summaries(grid.splitlines())
        part1 += summaries[0]

        for c in range(len(grid)):
            if grid[c] == "\n":
                continue

            corrected_grid = (grid[:c] + ("#" if grid[c] == "." else ".") + grid[c + 1 :]).splitlines()
            corrected_summaries = [summary for summary in get_summaries(corrected_grid) if summary != summaries[0]]

            if len(corrected_summaries) > 0:
                part2 += corrected_summaries[0]
                break

    return part1, part2


def get_summaries(grid: list) -> list:
    summaries = []

    for i in range(1, len(grid)):
        left = grid[:i]
        right = grid[i:]
        size = min(len(left), len(right))
        if list(reversed(left))[:size] == right[:size]:
            summaries.append(100 * i)

    grid = list(list(row) for row in zip(*grid[::-1]))
    for i in range(1, len(grid)):
        left = grid[:i]
        right = grid[i:]
        size = min(len(left), len(right))
        if list(reversed(left))[:size] == right[:size]:
            summaries.append(i)

    return summaries


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
        assert part1 == 405
        assert part2 == 400
