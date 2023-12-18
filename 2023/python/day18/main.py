import os

day = os.path.dirname(os.path.realpath(__file__))

directions = {"U": (0, -1), "D": (0, 1), "L": (-1, 0), "R": (1, 0)}


def part1(file: str):
    trench = set()
    cur = (0, 0)

    for line in [line.split() for line in file.splitlines()]:
        cur = add(cur, multiply(directions[line[0]], int(line[1])))
        trench.add(cur)

    return calculate_area(trench)


def part2(file: str):
    trench = set()
    cur = (0, 0)

    for line in [line.split() for line in file.splitlines()]:
        cur = add(cur, multiply(directions["RDLU"[int(line[2][-2])]], int(line[2][2:-2], 16)))
        trench.add(cur)

    return calculate_area(trench)


def calculate_area(shape):
    area = 0
    edges = []

    last_y = None
    line_area = 0

    for y in sorted(list(set([space[1] for space in shape]))):
        if last_y is not None:
            area += line_area * (y - last_y - 1)
        last_y = y

        vertices = [point for point in shape if point[1] == y]
        vertices.sort(key=lambda point: point[0])

        ranges = [[edges[i], edges[i + 1]] for i in range(0, len(edges), 2)]

        for vertex in vertices:
            try:
                edges.remove(vertex[0])
            except:
                edges.append(vertex[0])
        edges.sort()

        new_ranges = [[edges[i], edges[i + 1]] for i in range(0, len(edges), 2)]
        ranges += new_ranges

        line_area = sum([r[1] - r[0] + 1 for r in merge_ranges(new_ranges)])
        area += sum([r[1] - r[0] + 1 for r in merge_ranges(ranges)])

    return area


def merge_ranges(ranges):
    ranges.sort(key=lambda r: r[0])
    merged = []
    for r in ranges:
        if not merged or merged[-1][1] < r[0]:
            merged.append(r)
            continue
        merged[-1][1] = max(merged[-1][1], r[1])
    return merged


def add(a: (int, int), b: (int, int)) -> (int, int):
    return (a[0] + b[0], a[1] + b[1])


def multiply(a: (int, int), b: int) -> (int, int):
    return (a[0] * b, a[1] * b)


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content) == 62
        assert part2(content) == 952408144115
