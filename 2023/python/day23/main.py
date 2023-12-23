import os
from collections import defaultdict

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    trails = file.splitlines()
    slopeless_trails = file.replace("^", ".").replace(">", ".").replace("v", ".").replace("<", ".").splitlines()

    graph = build_graph(trails)
    slopeless_graph = build_graph(slopeless_trails)

    simplified_graph = build_simplified_graph(graph, slopeless_graph)
    slopeless_simplified_graph = build_simplified_graph(slopeless_graph, slopeless_graph)

    start = (trails[0].index("."), 0)
    end = (trails[len(trails) - 1].index("."), len(trails) - 1)

    max_length = longest_path(simplified_graph, start, end)
    slopeless_max_length = longest_path(slopeless_simplified_graph, start, end)

    return max_length, slopeless_max_length


def build_graph(trails: list):
    graph = defaultdict(dict)

    for y, line in enumerate(trails):
        for x in [x for x, c in enumerate(line) if c != "#"]:
            for dx, dy, slope in [(0, -1, "^"), (1, 0, ">"), (0, 1, "v"), (-1, 0, "<")]:
                if 0 <= x + dx < len(line) and 0 <= y + dy < len(trails):
                    c = trails[y + dy][x + dx]
                    if c == "." or c == slope:
                        graph[(x, y)][((x + dx, y + dy))] = 1

    return graph


def build_simplified_graph(graph, slopeless_graph):
    simplified_graph = {node: {} for node, neighbors in slopeless_graph.items() if len(neighbors) != 2}

    for node in simplified_graph.keys():
        adjacent = find_adjacent_intersections(graph, simplified_graph, node)
        for n in adjacent.keys():
            simplified_graph[node][n] = adjacent[n]

    return simplified_graph


def find_adjacent_intersections(old_graph, new_graph, node):
    adjacent = {}

    for n in old_graph[node].keys():
        path = [n, node]
        while path[0] not in new_graph.keys():
            neighbor = next((n for n in list(old_graph[path[0]].keys()) if n not in path), None)
            if neighbor is None:
                break
            path.insert(0, neighbor)
        else:
            adjacent[path[0]] = len(path) - 1

    return adjacent


def longest_path(graph, start, end, visited=set(), path=None):
    if path is None:
        path = [start]

    visited.add(start)
    if start == end:
        return 0

    return max(
        [
            longest_path(graph, node, end, visited.copy(), path + [node]) + weight
            for node, weight in graph[start].items()
            if node not in visited
        ],
        default=0,
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
        assert part1 == 94
        assert part2 == 154
