package io.mchill.day21;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.PriorityQueue;
import java.util.stream.Collectors;

import org.apache.commons.lang3.tuple.Pair;

public class Solution {
    private static HashMap<Pair<String, Integer>, Long> cache = new HashMap<>();

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

    private static Long part1(List<String> input) {
        return solve(input, 3);
    }

    private static Long part2(List<String> input) {
        return solve(input, 26);
    }

    private static Long solve(List<String> input, Integer robots) {
        Map<Character, Button> keypad = buildKeypad();

        Long sum = 0L;

        for (String numericKeypadPath : input) {
            Long pathLength = getPathLength(keypad, 'A' + numericKeypadPath, robots);
            Integer numericCode = Integer.parseInt(numericKeypadPath.substring(0, numericKeypadPath.length() - 1));
            sum += pathLength * numericCode;
        }

        return sum;
    }

    private static Long getPathLength(Map<Character, Button> keypad, String path, Integer levels) {
        if (cache.containsKey(Pair.of(path, levels))) {
            return cache.get(Pair.of(path, levels));
        }

        if (levels == 0) {
            return (long) path.length() - 1;
        }

        Long length = 0L;
        for (int i = 1; i < path.length(); i++) {
            List<String> innerPaths = findShortestPaths(keypad.get(path.charAt(i - 1)), keypad.get(path.charAt(i)));
            Long innerPathLength = Long.MAX_VALUE;
            for (String innerPath : innerPaths) {
                innerPathLength = Math.min(innerPathLength, getPathLength(keypad, 'B' + innerPath + 'B', levels - 1));
            }
            length += innerPathLength;
        }

        cache.put(Pair.of(path, levels), length);
        return length;
    }

    private static Map<Character, Button> buildKeypad() {
        Button up = new Button('^');
        Button b = new Button('B', Map.ofEntries(Map.entry('<', up)));
        Button left = new Button('<');
        Button down = new Button('v', Map.ofEntries(Map.entry('^', up), Map.entry('<', left)));
        Button right = new Button('>', Map.ofEntries(Map.entry('^', b), Map.entry('<', down)));

        Button seven = new Button('7');
        Button eight = new Button('8', Map.ofEntries(Map.entry('<', seven)));
        Button nine = new Button('9', Map.ofEntries(Map.entry('<', eight)));
        Button four = new Button('4', Map.ofEntries(Map.entry('^', seven)));
        Button five = new Button('5', Map.ofEntries(Map.entry('^', eight), Map.entry('<', four)));
        Button six = new Button('6', Map.ofEntries(Map.entry('^', nine), Map.entry('<', five)));
        Button one = new Button('1', Map.ofEntries(Map.entry('^', four)));
        Button two = new Button('2', Map.ofEntries(Map.entry('^', five), Map.entry('<', one)));
        Button three = new Button('3', Map.ofEntries(Map.entry('^', six), Map.entry('<', two)));
        Button zero = new Button('0', Map.ofEntries(Map.entry('^', two)));
        Button a = new Button('A', Map.ofEntries(Map.entry('^', three), Map.entry('<', zero)));

        return List.of(up, b, left, down, right, seven, eight, nine, four, five, six, one, two, three, zero, a).stream()
                .collect(Collectors.toMap(button -> button.name, button -> button));
    }

    private static List<String> findShortestPaths(Button start, Button end) {
        PriorityQueue<Node> open = new PriorityQueue<>();
        Map<Button, Integer> closed = new HashMap<>();
        List<String> shortestPaths = new ArrayList<>();
        int shortestPathLength = Integer.MAX_VALUE;

        Node startNode = new Node(start, "", 0);
        open.add(startNode);

        while (!open.isEmpty()) {
            Node current = open.poll();

            if (current.button.equals(end)) {
                if (current.score < shortestPathLength) {
                    shortestPathLength = current.score;
                    shortestPaths.clear();
                    shortestPaths.add(current.path);
                } else if (current.score == shortestPathLength) {
                    shortestPaths.add(current.path);
                }
            }

            closed.put(current.button, current.score);

            for (Node neighbor : current.getNeighbors()) {
                if (!closed.containsKey(neighbor.button) || neighbor.score <= closed.get(neighbor.button)) {
                    open.add(neighbor);
                }
            }
        }

        return shortestPaths;
    }

    private static class Node implements Comparable<Node> {
        Button button;
        Integer score;
        String path;

        Node(Button button, String path, Integer score) {
            this.button = button;
            this.score = score;
            this.path = path;
        }

        @Override
        public int compareTo(Node other) {
            return score - other.score;
        }

        public List<Node> getNeighbors() {
            List<Node> neighbors = new ArrayList<>();

            for (Entry<Character, Button> entry : button.neighbors.entrySet()) {
                Character neighborDirection = entry.getKey();
                Button neighborButton = entry.getValue();

                String path = this.path + neighborDirection;

                neighbors.add(new Node(neighborButton, path, score + 1));
            }

            return neighbors;
        }
    }

    private static class Button {
        public Character name;
        public HashMap<Character, Button> neighbors = new HashMap<>();

        public Button(Character name) {
            this.name = name;
            this.neighbors = new HashMap<>();
        }

        public Button(Character name, Map<Character, Button> neighbors) {
            Map<Character, Character> oppositeDirections = Map.ofEntries(
                    Map.entry('>', '<'),
                    Map.entry('v', '^'),
                    Map.entry('<', '>'),
                    Map.entry('^', 'v'));

            this.name = name;
            this.neighbors = new HashMap<>(neighbors);
            for (Entry<Character, Button> entry : neighbors.entrySet()) {
                Character direction = entry.getKey();
                Button neighbor = entry.getValue();
                neighbor.neighbors.put(oppositeDirections.get(direction), this);
            }
        }
    }
}
