package io.mchill.day10;

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
        int[][] map = pad2DList(input).stream().map(line -> line.chars().map(Character::getNumericValue).toArray())
                .toArray(int[][]::new);

        List<Pair<Integer, Integer>> trailheads = getTrailheads(map);

        Integer total = 0;
        for (Pair<Integer, Integer> trailhead : trailheads) {
            total += getTrailheadScore(map, trailhead);
        }
        return total;
    }

    private static Integer part2(List<String> input) {
        int[][] map = pad2DList(input).stream().map(line -> line.chars().map(Character::getNumericValue).toArray())
                .toArray(int[][]::new);

        List<Pair<Integer, Integer>> trailheads = getTrailheads(map);

        Integer total = 0;
        for (Pair<Integer, Integer> trailhead : trailheads) {
            total += getTrailheadRating(map, trailhead);
        }
        return total;
    }

    private static List<String> pad2DList(List<String> input) {
        List<String> lines = new ArrayList<String>(input);
        String padding = ".".repeat(lines.get(0).length());
        lines.add(0, padding);
        lines.add(padding);
        return lines.stream().map(line -> ("." + line + ".")).toList();
    }

    private static List<Pair<Integer, Integer>> getTrailheads(int[][] map) {
        List<Pair<Integer, Integer>> trailheads = new ArrayList<>();
        for (int y = 0; y < map.length; y++) {
            for (int x = 0; x < map[y].length; x++) {
                if (map[y][x] == 0) {
                    trailheads.add(Pair.of(x, y));
                }
            }
        }
        return trailheads;
    }

    private static Integer getTrailheadScore(int[][] map, Pair<Integer, Integer> trailhead) {
        HashSet<Pair<Integer, Integer>> peaks = new HashSet<>();

        hike(map, trailhead, peaks);

        return peaks.size();
    }

    private static Integer getTrailheadRating(int[][] map, Pair<Integer, Integer> trailhead) {
        HashSet<Pair<Integer, Integer>> peaks = new HashSet<>();

        return hike(map, trailhead, peaks);
    }

    private static Integer hike(int[][] map, Pair<Integer, Integer> position, HashSet<Pair<Integer, Integer>> peaks) {
        Integer height = height(map, position);

        if (height == 9) {
            peaks.add(position);
            return 1;
        }

        Integer totalPeaks = 0;

        Pair<Integer, Integer> east = Pair.of(position.getLeft() + 1, position.getRight());
        Pair<Integer, Integer> south = Pair.of(position.getLeft(), position.getRight() + 1);
        Pair<Integer, Integer> west = Pair.of(position.getLeft() - 1, position.getRight());
        Pair<Integer, Integer> north = Pair.of(position.getLeft(), position.getRight() - 1);

        if (height(map, east) == height + 1) {
            totalPeaks += hike(map, east, peaks);
        }
        if (height(map, south) == height + 1) {
            totalPeaks += hike(map, south, peaks);
        }
        if (height(map, west) == height + 1) {
            totalPeaks += hike(map, west, peaks);
        }
        if (height(map, north) == height + 1) {
            totalPeaks += hike(map, north, peaks);
        }

        return totalPeaks;
    }

    private static Integer height(int[][] map, Pair<Integer, Integer> position) {
        return map[position.getRight()][position.getLeft()];
    }
}
