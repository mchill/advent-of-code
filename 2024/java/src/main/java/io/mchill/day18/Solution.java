package io.mchill.day18;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;

import org.apache.commons.lang3.tuple.Pair;

public class Solution {
    final static Integer SIZE = 71;

    public static void main(String[] args) throws IOException {
        List<String> input = readInput(args[0]);
        System.out.println("Part 1: " + part1(input));
        System.out.println("Part 2: " + part2(input));
    }

    private static List<String> readInput(String path) throws IOException {
        String packagePath = Solution.class.getPackageName().replace('.', '/');
        String fullPath = Paths.get("src/main/java", packagePath, path).toString();
        return Files.readAllLines(Paths.get(fullPath));
    }

    private static Integer part1(List<String> input) {
        char[][] map = createMap(input, 1024);
        return findShortestPath(map, new HashSet<>());
    }

    private static String part2(List<String> input) {
        char[][] map = createMap(input, 0);

        for (int nanosecond = 0; true; nanosecond++) {
            addBit(map, input, nanosecond);
            Integer path = findShortestPath(map, new HashSet<>());
            if (path == Integer.MAX_VALUE) {
                return input.get(nanosecond);
            }
        }
    }

    private static char[][] createMap(List<String> input, Integer nanoseconds) {
        char[][] map = new char[SIZE + 2][SIZE + 2];

        for (int y = 0; y < SIZE + 2; y++) {
            for (int x = 0; x < SIZE + 2; x++) {
                map[y][x] = (x == 0 || x == SIZE + 1 || y == 0 || y == SIZE + 1) ? '#' : '.';
            }
        }

        for (int nanosecond = 0; nanosecond < nanoseconds; nanosecond++) {
            addBit(map, input, nanosecond);
        }

        return map;
    }

    private static void addBit(char[][] map, List<String> input, Integer nanosecond) {
        String[] parts = input.get(nanosecond).split(",");
        map[Integer.parseInt(parts[1]) + 1][Integer.parseInt(parts[0]) + 1] = '#';
    }

    private static Integer findShortestPath(char[][] map, HashSet<Pair<Integer, Integer>> visited) {
        Pair<Integer, Integer> startPosition = Pair.of(1, 1);
        Pair<Integer, Integer> endPosition = Pair.of(SIZE, SIZE);

        PriorityQueue<Node> open = new PriorityQueue<>(new Comparator<Node>() {
            public int compare(Node a, Node b) {
                return a.score - b.score;
            }
        });
        Map<Pair<Integer, Integer>, Node> openMap = new HashMap<>();
        Set<Pair<Integer, Integer>> closed = new HashSet<>();

        Node startNode = new Node(startPosition, new ArrayList<>(), 0);
        open.add(startNode);
        openMap.put(startPosition, startNode);

        Integer shortest = Integer.MAX_VALUE;

        while (!open.isEmpty()) {
            Node current = open.poll();
            openMap.remove(current.position);
            closed.add(current.position);

            if (current.position.equals(endPosition)) {
                for (Node node : current.path) {
                    visited.add(node.position);
                }
                shortest = Math.min(shortest, current.score);
                continue;
            }

            for (Node neighbor : current.getNeighbors()) {
                if (map[neighbor.position.getRight()][neighbor.position.getLeft()] == '#'
                        || closed.contains(neighbor.position)) {
                    continue;
                }

                Node existing = openMap.get(neighbor.position);

                if (existing == null || neighbor.score <= existing.score) {
                    open.add(neighbor);
                    open.remove(existing);
                    openMap.put(neighbor.position, neighbor);
                }
            }
        }

        return shortest;
    }

    private static class Node {
        Pair<Integer, Integer> position;
        List<Node> path;
        Integer score;

        public Node(Pair<Integer, Integer> position, List<Node> path, Integer score) {
            this.position = position;
            this.path = path;
            this.score = score;
        }

        public List<Node> getNeighbors() {
            List<Node> neighbors = new ArrayList<>();

            List<Node> newPath = new ArrayList<>(path);
            newPath.add(this);

            neighbors.add(new Node(Pair.of(position.getLeft() + 1, position.getRight()), newPath, score + 1));
            neighbors.add(new Node(Pair.of(position.getLeft(), position.getRight() + 1), newPath, score + 1));
            neighbors.add(new Node(Pair.of(position.getLeft() - 1, position.getRight()), newPath, score + 1));
            neighbors.add(new Node(Pair.of(position.getLeft(), position.getRight() - 1), newPath, score + 1));

            return neighbors;
        }
    }
}
