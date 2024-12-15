package io.mchill.day15;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.Map;

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
        String[] parts = input.split("\n\n");
        char[][] map = getMap(parts[0]);
        Pair<Integer, Integer> robot = getStart(map);
        String moves = parts[1].replace("\n", "");

        for (char move : moves.toCharArray()) {
            Pair<Integer, Integer> direction = Map.ofEntries(
                    Map.entry('>', Pair.of(1, 0)),
                    Map.entry('v', Pair.of(0, 1)),
                    Map.entry('<', Pair.of(-1, 0)),
                    Map.entry('^', Pair.of(0, -1))).get(move);

            if (canMove(map, robot, direction)) {
                move(map, robot, direction);
                robot = addPair(robot, direction);
            }
        }

        return getGPSCoordinates(map);
    }

    private static Integer part2(String input) {
        String[] parts = input.split("\n\n");
        char[][] map = getMap(parts[0].replace("#", "##").replace("O", "[]").replace(".", "..").replace("@", "@."));
        Pair<Integer, Integer> robot = getStart(map);
        String moves = parts[1].replace("\n", "");

        for (char move : moves.toCharArray()) {
            Pair<Integer, Integer> direction = Map.ofEntries(
                    Map.entry('>', Pair.of(1, 0)),
                    Map.entry('v', Pair.of(0, 1)),
                    Map.entry('<', Pair.of(-1, 0)),
                    Map.entry('^', Pair.of(0, -1))).get(move);

            if (canMove(map, robot, direction)) {
                move(map, robot, direction);
                robot = addPair(robot, direction);
            }
        }

        return getGPSCoordinates(map);
    }

    private static char[][] getMap(String input) {
        return Arrays.stream(input.split("\n"))
                .map(String::toCharArray)
                .toArray(char[][]::new);
    }

    private static Pair<Integer, Integer> getStart(char[][] map) {
        Integer robotY = Arrays.stream(map)
                .filter(row -> new String(row).contains("@"))
                .map(row -> Arrays.asList(map).indexOf(row))
                .findFirst()
                .orElseThrow();
        Integer robotX = String.valueOf(map[robotY]).indexOf('@');
        return Pair.of(robotX, robotY);
    }

    private static Boolean canMove(char[][] map, Pair<Integer, Integer> position, Pair<Integer, Integer> direction) {
        Pair<Integer, Integer> newPosition = addPair(position, direction);
        char next = getChar(map, newPosition);

        if (next == '#') {
            return false;
        }
        if (next == '.') {
            return true;
        }

        if (direction.getLeft() != 0) {
            return canMove(map, newPosition, direction);
        }

        return switch (next) {
            case 'O' -> canMove(map, newPosition, direction);
            case '[' ->
                canMove(map, newPosition, direction) && canMove(map, addPair(newPosition, Pair.of(1, 0)), direction);
            case ']' -> canMove(map, newPosition, direction)
                    && canMove(map, addPair(newPosition, Pair.of(-1, 0)), direction);
            default -> throw new IllegalArgumentException("Invalid character: " + next);
        };
    }

    private static void move(char[][] map, Pair<Integer, Integer> position,
            Pair<Integer, Integer> direction) {
        Pair<Integer, Integer> newPosition = addPair(position, direction);

        if (getChar(map, newPosition) == 'O') {
            move(map, newPosition, direction);
        }

        if (getChar(map, newPosition) == '[') {
            move(map, newPosition, direction);
            if (direction.getRight() != 0) {
                move(map, addPair(newPosition, Pair.of(1, 0)), direction);
            }
        }

        if (getChar(map, newPosition) == ']') {
            move(map, newPosition, direction);
            if (direction.getRight() != 0) {
                move(map, addPair(newPosition, Pair.of(-1, 0)), direction);
            }
        }

        if (getChar(map, newPosition) == '.') {
            map[newPosition.getRight()][newPosition.getLeft()] = getChar(map, position);
            map[position.getRight()][position.getLeft()] = '.';
        }
    }

    private static char getChar(char[][] map, Pair<Integer, Integer> position) {
        return map[position.getRight()][position.getLeft()];
    }

    private static Pair<Integer, Integer> addPair(Pair<Integer, Integer> a, Pair<Integer, Integer> b) {
        return Pair.of(a.getLeft() + b.getLeft(), a.getRight() + b.getRight());
    }

    private static Integer getGPSCoordinates(char[][] map) {
        Integer total = 0;
        for (int y = 0; y < map.length; y++) {
            for (int x = 0; x < map[y].length; x++) {
                if (map[y][x] == 'O' || map[y][x] == '[') {
                    total += y * 100 + x;
                }
            }
        }
        return total;
    }
}
