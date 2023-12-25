import os
from functools import reduce
from operator import mul

import networkx as nx

day = os.path.dirname(os.path.realpath(__file__))


def part1(file: str):
    graph = nx.Graph()
    for node, connected_nodes in [line.split(": ") for line in file.splitlines()]:
        graph.add_edges_from([(node, connected) for connected in connected_nodes.split(" ")])
    graph.remove_edges_from(nx.algorithms.connectivity.minimum_edge_cut(graph))
    return reduce(mul, [len(c) for c in nx.connected_components(graph)])


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content) == 54
