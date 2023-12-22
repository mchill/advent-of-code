import os
from collections import defaultdict

day = os.path.dirname(os.path.realpath(__file__))


def part1(file: str):
    bricks = get_bricks(file)
    space = get_space(bricks)
    settle(space, bricks)
    supports, supported_by = get_supports(space, bricks)

    return sum(1 for i in range(len(bricks)) if all(len(supported_by[other_brick]) > 1 for other_brick in supports[i]))


def part2(file: str):
    bricks = get_bricks(file)
    space = get_space(bricks)
    settle(space, bricks)
    supports, supported_by = get_supports(space, bricks)

    return sum(disintegrate(supports, supported_by, i) for i in range(len(bricks)))


def disintegrate(supports, supported_by, brick_index):
    fallen = {brick_index}
    nodes = list(supports[brick_index])

    while nodes:
        cur = nodes.pop(0)
        if not all(other_brick in fallen for other_brick in supported_by[cur]):
            continue
        fallen.add(cur)
        nodes.extend(node for node in supports[cur] if node not in fallen)

    return len(fallen) - 1


def get_bricks(file: str):
    return sorted(
        [[list(map(int, point.split(","))) for point in line.split("~")] for line in file.splitlines()],
        key=lambda brick: min(brick[0][2], brick[1][2]),
    )


def get_space(bricks):
    space = {}

    for i, [p1, p2] in enumerate(bricks):
        for x in range(p1[0], p2[0] + 1):
            for y in range(p1[1], p2[1] + 1):
                for z in range(p1[2], p2[2] + 1):
                    space[(x, y, z)] = i

    return space


def can_move_down(space, brick, brick_index):
    [p1, p2] = brick

    if min(p1[2], p2[2]) == 1:
        return False

    for x in range(p1[0], p2[0] + 1):
        for y in range(p1[1], p2[1] + 1):
            for z in range(p1[2], p2[2] + 1):
                if (x, y, z - 1) in space and space[(x, y, z - 1)] != brick_index:
                    return False

    return True


def settle(space, bricks):
    for brick_index, [p1, p2] in enumerate(bricks):
        while can_move_down(space, [p1, p2], brick_index):
            for x in range(p1[0], p2[0] + 1):
                for y in range(p1[1], p2[1] + 1):
                    for z in range(p1[2], p2[2] + 1):
                        del space[(x, y, z)]
                        space[(x, y, z - 1)] = brick_index

            p1[2] -= 1
            p2[2] -= 1


def get_supports(space, bricks):
    supports = defaultdict(set)
    supported_by = defaultdict(set)

    for brick_index, [p1, p2] in enumerate(bricks):
        for x in range(p1[0], p2[0] + 1):
            for y in range(p1[1], p2[1] + 1):
                for z in range(p1[2], p2[2] + 1):
                    if (x, y, z + 1) in space and space[(x, y, z + 1)] != brick_index:
                        supports[brick_index].add(space[(x, y, z + 1)])
                    if (x, y, z - 1) in space and space[(x, y, z - 1)] != brick_index:
                        supported_by[brick_index].add(space[(x, y, z - 1)])

    return supports, supported_by


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content) == 5
        assert part2(content) == 7
