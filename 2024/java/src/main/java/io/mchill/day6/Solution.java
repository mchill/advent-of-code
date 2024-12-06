package io.mchill.day6;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

import org.apache.commons.lang3.tuple.Pair;

public class Solution {
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
        List<ArrayList<Character>> map = getMap(input);

        HashSet<Pair<Integer, Integer>> visited = new HashSet<>();
        isLoop(map, visited);

        return visited.size();
    }

    private static Integer part2(List<String> input) {
        List<ArrayList<Character>> map = getMap(input);

        HashSet<Pair<Integer, Integer>> visited = new HashSet<>();
        List<ArrayList<Character>> copy = map.stream().map(ArrayList::new).toList();
        isLoop(copy, visited);

        Integer loops = 0;

        for (Pair<Integer, Integer> position : visited) {
            Integer x = position.getLeft();
            Integer y = position.getRight();

            if (map.get(y).get(x) == '^') {
                continue;
            }

            copy = map.stream().map(ArrayList::new).toList();
            copy.get(y).set(x, '#');

            if (isLoop(copy, new HashSet<>())) {
                loops += 1;
            }
        }

        return loops;
    }

    private static List<ArrayList<Character>> getMap(List<String> input) {
        return input.stream()
                .map(row -> new ArrayList<>(row.chars().mapToObj(c -> (char) c).toList())).toList();
    }

    private static Boolean isLoop(List<ArrayList<Character>> map, HashSet<Pair<Integer, Integer>> visited) {
        Integer y = map.stream().filter(row -> row.contains('^')).map(map::indexOf).findFirst().orElseThrow();
        Integer x = map.get(y).indexOf('^');

        HashSet<String> states = new HashSet<>();
        String state = String.format("%c,%d,%d", '^', x, y);

        while (true) {
            if (states.contains(state)) {
                return true;
            }
            states.add(state);
            visited.add(Pair.of(x, y));

            Character direction = map.get(y).get(x);

            Integer nextX = x + (direction == '>' ? 1 : direction == '<' ? -1 : 0);
            Integer nextY = y + (direction == 'v' ? 1 : direction == '^' ? -1 : 0);

            if (nextX < 0 || nextX >= map.get(0).size() || nextY < 0 || nextY >= map.size()) {
                return false;
            }

            Character nextChar = map.get(nextY).get(nextX);

            if (nextChar == '#') {
                List<Character> directions = List.of('>', 'v', '<', '^');
                direction = directions.get((directions.indexOf(direction) + 1) % 4);
                nextX = x;
                nextY = y;
            }

            map.get(y).set(x, 'X');
            map.get(nextY).set(nextX, direction);

            x = nextX;
            y = nextY;

            state = String.format("%c,%d,%d", direction, x, y);
        }
    }
}
