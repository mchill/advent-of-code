package io.mchill.day16;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;

import org.apache.commons.lang3.tuple.Pair;

public class Solution {
    public static void main(String[] args) throws IOException {
        String input = readInput(args[0]);
        System.out.println("Part 1: " + part1(input));
        System.out.println("Part 2: " + part2(input));
    }

    private static String readInput(String path) throws IOException {
        String packagePath = Solution.class.getPackageName().replace('.', '/');
        String fullPath = Paths.get("src/main/java", packagePath, path).toString();
        return Files.readString(Paths.get(fullPath));
    }

    private static Integer part1(String input) {
        return findShortestPath(input, new HashSet<>());
    }

    private static Integer part2(String input) {
        HashSet<Pair<Integer, Integer>> visited = new HashSet<>();
        findShortestPath(input, visited);
        return visited.size() + 1;
    }

    private static Integer findShortestPath(String input, HashSet<Pair<Integer, Integer>> visited) {
        char[][] map = input.lines().map(String::toCharArray).toArray(char[][]::new);
        Pair<Integer, Integer> startPosition = getPosition(map, 'S');
        Pair<Integer, Integer> endPosition = getPosition(map, 'E');
        Pair<Integer, Integer> startDirection = Pair.of(1, 0);

        PriorityQueue<Node> open = new PriorityQueue<>(new Comparator<Node>() {
            public int compare(Node a, Node b) {
                return a.score - b.score;
            }
        });
        Map<Pair<Pair<Integer, Integer>, Pair<Integer, Integer>>, Node> openMap = new HashMap<>();
        Set<Pair<Pair<Integer, Integer>, Pair<Integer, Integer>>> closed = new HashSet<>();

        Node startNode = new Node(startPosition, startDirection, new ArrayList<>(), 0);
        open.add(startNode);
        openMap.put(Pair.of(startPosition, startDirection), startNode);

        Integer shortest = Integer.MAX_VALUE;

        while (!open.isEmpty()) {
            Node current = open.poll();
            openMap.remove(Pair.of(current.position, current.direction));
            closed.add(Pair.of(current.position, current.direction));

            if (current.position.equals(endPosition)) {
                for (Node node : current.path) {
                    visited.add(node.position);
                }
                shortest = Math.min(shortest, current.score);
                continue;
            }

            for (Node neighbor : current.getNeighbors()) {
                if (map[neighbor.position.getRight()][neighbor.position.getLeft()] == '#'
                        || closed.contains(Pair.of(neighbor.position, neighbor.direction))) {
                    continue;
                }

                Node existing = openMap.get(Pair.of(neighbor.position, neighbor.direction));

                if (existing == null || neighbor.score <= existing.score) {
                    open.add(neighbor);
                    openMap.put(Pair.of(neighbor.position, neighbor.direction), neighbor);
                }
            }
        }

        return shortest;
    }

    private static Pair<Integer, Integer> getPosition(char[][] map, char c) {
        Integer y = Arrays.stream(map)
                .filter(row -> new String(row).indexOf(c) > -1)
                .map(row -> Arrays.asList(map).indexOf(row))
                .findFirst()
                .orElseThrow();
        Integer x = String.valueOf(map[y]).indexOf(c);
        return Pair.of(x, y);
    }

    private static class Node {
        Pair<Integer, Integer> position;
        Pair<Integer, Integer> direction;
        List<Node> path;
        Integer score;

        public Node(Pair<Integer, Integer> position, Pair<Integer, Integer> direction, List<Node> path, Integer score) {
            this.position = position;
            this.direction = direction;
            this.path = path;
            this.score = score;
        }

        public List<Node> getNeighbors() {
            List<Node> neighbors = new ArrayList<>();

            List<Node> newPath = new ArrayList<>(path);
            newPath.add(this);

            neighbors.add(new Node(
                    Pair.of(position.getLeft() + direction.getLeft(), position.getRight() + direction.getRight()),
                    direction, newPath, score + 1));
            neighbors
                    .add(new Node(position, Pair.of(direction.getRight(), direction.getLeft()), newPath, score + 1000));
            neighbors.add(
                    new Node(position, Pair.of(-direction.getRight(), -direction.getLeft()), newPath, score + 1000));

            return neighbors;
        }

        public String toString() {
            return String.format("Node(%s, %s, %d)", position, direction, score);
        }
    }
}
