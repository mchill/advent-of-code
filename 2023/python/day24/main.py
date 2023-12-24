import itertools
import os

day = os.path.dirname(os.path.realpath(__file__))


def part1(file: str, boundary: tuple):
    stones = []
    for left, right in [line.split(" @ ") for line in file.splitlines()]:
        position = tuple([int(p) for p in left.split(", ")])
        velocity = tuple([int(v) for v in right.split(", ")])
        stones.append((position, velocity))

    part1 = 0

    for stone1, stone2 in itertools.combinations(stones, 2):
        p1, v1 = stone1
        p2, v2 = stone2

        intersection = calculate_intersection(p1, v1, p2, v2)
        if not intersection:
            continue
        (x, y) = intersection

        if (
            (v1[0] <= 0 or x >= p1[0])
            and (v1[0] >= 0 or x <= p1[0])
            and (v2[0] <= 0 or x >= p2[0])
            and (v2[0] >= 0 or x <= p2[0])
            and boundary[0] <= x <= boundary[1]
            and boundary[0] <= y <= boundary[1]
        ):
            part1 += 1

    return part1


def part2(file: str):
    stones = []
    for left, right in [line.split(" @ ") for line in file.splitlines()]:
        position = tuple([int(p) for p in left.split(", ")])
        velocity = tuple([int(v) for v in right.split(", ")])
        stones.append((position, velocity))

    possible_velocities = [[], [], []]
    velocity_range = (
        min([v[0] for _, v in stones] + [v[1] for _, v in stones]),
        max([v[0] for _, v in stones] + [v[1] for _, v in stones]),
    )

    for stone1, stone2 in itertools.combinations(stones, 2):
        p1, v1 = stone1
        p2, v2 = stone2

        for i in range(3):
            if v1[i] == v2[i]:
                distance = p1[i] - p2[i]
                possibilities = set()
                for j in range(velocity_range[0], velocity_range[1]):
                    if j != v1[i] and distance % (j - v1[i]) == 0:
                        possibilities.add(j)
                possible_velocities[i].append(possibilities)

    rock_velocity = tuple([set.intersection(*possible_velocities[i]).pop() for i in range(3)])

    p1, v1 = stones[0]
    p2, v2 = stones[1]

    x, y = calculate_intersection(p1, subtract(v1, rock_velocity), p2, subtract(v2, rock_velocity))
    t = (x - p1[0]) // (v1[0] - rock_velocity[0])
    z = p1[2] + (v1[2] - rock_velocity[2]) * t

    return x + y + z


def calculate_intersection(p1, v1, p2, v2):
    m1 = v1[1] / v1[0]
    m2 = v2[1] / v2[0]

    if m1 == m2:
        return None

    c1 = p1[1] - (m1 * p1[0])
    c2 = p2[1] - (m2 * p2[0])

    x = int((c2 - c1) / (m1 - m2))
    y = int(m1 * x + c1)

    return (x, y)


def subtract(v1, v2):
    return tuple([v1[i] - v2[i] for i in range(3)])


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content, (200000000000000, 400000000000000))))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content, (7, 27)) == 2
