import heapq
import math
import os
from collections import defaultdict

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    city = [[int(number) for number in line] for line in file.splitlines()]
    neighbors = {}
    for y, row in enumerate(file.splitlines()):
        for x in range(len(row)):
            north_neighbor = (x, y - 1, "S") if y > 0 else None
            east_neighbor = (x + 1, y, "W") if x < len(row) - 1 else None
            south_neighbor = (x, y + 1, "N") if y < len(city) - 1 else None
            west_neighbor = (x - 1, y, "E") if x > 0 else None

            neighbors[(x, y, "N")] = [south_neighbor, east_neighbor, west_neighbor]
            neighbors[(x, y, "E")] = [west_neighbor, south_neighbor, north_neighbor]
            neighbors[(x, y, "S")] = [north_neighbor, west_neighbor, east_neighbor]
            neighbors[(x, y, "W")] = [east_neighbor, north_neighbor, south_neighbor]

    return dijkstra(city, neighbors, 1, 3), dijkstra(city, neighbors, 4, 10)


def dijkstra(city: list, neighbors: dict, min_straight=0, max_straight=math.inf):
    distances = defaultdict(lambda: float("inf"))
    distances[((0, 0, "N"), 1)] = 0
    distances[((0, 0, "W"), 1)] = 0
    queue = [(0, (0, 0, "N")), (0, (0, 0, "W"))]

    while queue:
        current_distance, position = heapq.heappop(queue)

        for neighbor in [neighbor for neighbor in neighbors[position][1:] if neighbor is not None]:
            next_neighbor = neighbor
            distance = current_distance

            for straight in range(1, max_straight + 1):
                neighbor = next_neighbor
                if neighbor is None:
                    break

                next_neighbor = neighbors[neighbor][0]
                distance += city[neighbor[1]][neighbor[0]]

                if straight < min_straight:
                    continue
                if straight > max_straight:
                    break

                if distance >= distances[(neighbor, straight)]:
                    continue
                distances[(neighbor, straight)] = distance

                heapq.heappush(queue, (distance, neighbor))

    return min(
        [
            distance
            for ((x, y, _), straight), distance in distances.items()
            if x == len(city[0]) - 1 and y == len(city) - 1 and straight >= min_straight
        ]
    )


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
        assert part1 == 102
        assert part2 == 94
