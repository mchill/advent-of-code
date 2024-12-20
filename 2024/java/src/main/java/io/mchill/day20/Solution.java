package io.mchill.day20;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;

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
        return findCheats(input, 2);
    }

    private static Integer part2(String input) {
        return findCheats(input, 20);
    }

    private static Integer findCheats(String input, Integer maxCheatDistance) {
        List<Node> path = findShortestPath(input);
        HashMap<Pair<Integer, Integer>, Integer> distances = new HashMap<>();
        for (Node node : path) {
            distances.put(node.position, node.score);
        }

        Integer cheats = 0;

        for (Node node : path) {
            for (int xDiff = -maxCheatDistance; xDiff <= maxCheatDistance; xDiff++) {
                for (int yDiff = -maxCheatDistance; yDiff <= maxCheatDistance; yDiff++) {
                    Integer distanceToCheat = Math.abs(xDiff) + Math.abs(yDiff);
                    if (distanceToCheat > maxCheatDistance) {
                        continue;
                    }

                    Integer cheatDistance = distances
                            .get(Pair.of(node.position.getLeft() + xDiff, node.position.getRight() + yDiff));

                    if (cheatDistance != null && node.score + distanceToCheat <= cheatDistance - 100) {
                        cheats++;
                    }
                }
            }
        }

        return cheats;
    }

    private static List<Node> findShortestPath(String input) {
        char[][] map = input.lines().map(String::toCharArray).toArray(char[][]::new);
        Pair<Integer, Integer> startPosition = getPosition(map, 'S');
        Pair<Integer, Integer> endPosition = getPosition(map, 'E');

        PriorityQueue<Node> open = new PriorityQueue<>();
        Map<Pair<Integer, Integer>, Integer> closed = new HashMap<>();

        Node startNode = new Node(startPosition, new ArrayList<>(), 0);
        open.add(startNode);

        while (!open.isEmpty()) {
            Node current = open.poll();

            if (current.position.equals(endPosition)) {
                return current.path;
            }

            closed.put(current.position, current.score);

            for (Node neighbor : current.getNeighbors(map)) {
                if (!closed.containsKey(neighbor.position)) {
                    open.add(neighbor);
                }
            }
        }

        return null;
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

    private static class Node implements Comparable<Node> {
        Pair<Integer, Integer> position;
        List<Node> path;
        Integer score;

        public Node(Pair<Integer, Integer> position, List<Node> path, Integer score) {
            this.position = position;
            this.path = new ArrayList<>(path);
            this.path.add(this);
            this.score = score;
        }

        @Override
        public int compareTo(Node other) {
            return score - other.score;
        }

        public List<Node> getNeighbors(char[][] map) {
            List<Node> neighbors = new ArrayList<>();

            Node toEast = new Node(Pair.of(position.getLeft() + 1, position.getRight()), path, score + 1);
            Node toSouth = new Node(Pair.of(position.getLeft(), position.getRight() + 1), path, score + 1);
            Node toWest = new Node(Pair.of(position.getLeft() - 1, position.getRight()), path, score + 1);
            Node toNorth = new Node(Pair.of(position.getLeft(), position.getRight() - 1), path, score + 1);

            if (map[toEast.position.getRight()][toEast.position.getLeft()] != '#') {
                neighbors.add(toEast);
            }
            if (map[toSouth.position.getRight()][toSouth.position.getLeft()] != '#') {
                neighbors.add(toSouth);
            }
            if (map[toWest.position.getRight()][toWest.position.getLeft()] != '#') {
                neighbors.add(toWest);
            }
            if (map[toNorth.position.getRight()][toNorth.position.getLeft()] != '#') {
                neighbors.add(toNorth);
            }

            return neighbors;
        }
    }
}
