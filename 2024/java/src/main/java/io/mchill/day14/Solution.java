package io.mchill.day14;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

import org.apache.commons.lang3.tuple.Pair;

public class Solution {
    final static Integer WIDTH = 101;
    final static Integer HEIGHT = 103;

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
        HashMap<Integer, Integer> quadrants = new HashMap<>();

        for (String line : input) {
            Robot robot = new Robot(line);
            robot.move(100);

            if (robot.position.getLeft() < WIDTH / 2) {
                if (robot.position.getRight() < HEIGHT / 2) {
                    quadrants.put(1, quadrants.getOrDefault(1, 0) + 1);
                } else if (robot.position.getRight() > HEIGHT / 2) {
                    quadrants.put(3, quadrants.getOrDefault(3, 0) + 1);
                }
            } else if (robot.position.getLeft() > WIDTH / 2) {
                if (robot.position.getRight() < HEIGHT / 2) {
                    quadrants.put(2, quadrants.getOrDefault(2, 0) + 1);
                } else if (robot.position.getRight() > HEIGHT / 2) {
                    quadrants.put(4, quadrants.getOrDefault(4, 0) + 1);
                }
            }
        }

        return quadrants.values().stream().reduce(1, (a, b) -> a * b);
    }

    private static Integer part2(List<String> input) {
        final Integer WIDTH = 101;
        final Integer HEIGHT = 103;

        List<Robot> robots = new ArrayList<>();

        for (String line : input) {
            robots.add(new Robot(line));
        }

        int seconds = 0;
        while (true) {
            seconds++;

            char[][] grid = new char[HEIGHT][WIDTH];
            for (int y = 0; y < HEIGHT; y++) {
                for (int x = 0; x < WIDTH; x++) {
                    grid[y][x] = ' ';
                }
            }

            for (Robot robot : robots) {
                robot.move(1);
                grid[robot.position.getRight()][robot.position.getLeft()] = '█';
            }

            for (char[] row : grid) {
                if (new String(row).contains("████████")) {
                    return seconds;
                }
            }
        }
    }

    private static class Robot {
        Pair<Integer, Integer> position;
        Pair<Integer, Integer> velocity;

        public Robot(String input) {
            String[] parts = input.split(" |=");

            String[] numbers = parts[1].split(",");
            this.position = Pair.of(Integer.parseInt(numbers[0]), Integer.parseInt(numbers[1]));
            numbers = parts[3].split(",");
            this.velocity = Pair.of(Integer.parseInt(numbers[0]), Integer.parseInt(numbers[1]));
        }

        public void move(Integer seconds) {
            this.position = Pair.of(
                    Math.floorMod(this.position.getLeft() + this.velocity.getLeft() * seconds, WIDTH),
                    Math.floorMod(this.position.getRight() + this.velocity.getRight() * seconds, HEIGHT));
        }
    }
}
