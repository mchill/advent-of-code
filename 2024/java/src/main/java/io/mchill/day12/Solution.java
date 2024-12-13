package io.mchill.day12;

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
        List<HashSet<Pair<Integer, Integer>>> regions = getRegions(input);
        Integer price = 0;

        for (HashSet<Pair<Integer, Integer>> region : regions) {
            price += getRegionArea(region) * getRegionPerimeter(region);
        }

        return price;
    }

    private static Integer part2(List<String> input) {
        List<HashSet<Pair<Integer, Integer>>> regions = getRegions(input);
        Integer price = 0;

        for (HashSet<Pair<Integer, Integer>> region : regions) {
            price += getRegionArea(region) * getRegionSides(region);
        }

        return price;
    }

    private static List<HashSet<Pair<Integer, Integer>>> getRegions(List<String> input) {
        char[][] garden = input.stream().map(String::toCharArray).toArray(char[][]::new);

        List<HashSet<Pair<Integer, Integer>>> regions = new ArrayList<>();
        HashSet<Pair<Integer, Integer>> visited = new HashSet<>();

        for (int y = 0; y < garden.length; y++) {
            for (int x = 0; x < garden[y].length; x++) {
                Pair<Integer, Integer> position = Pair.of(x, y);

                if (visited.contains(position)) {
                    continue;
                }

                HashSet<Pair<Integer, Integer>> region = new HashSet<>();
                fillRegion(garden, region, garden[y][x], position);

                regions.add(region);
                visited.addAll(region);
            }
        }

        return regions;
    }

    private static void fillRegion(char[][] garden, HashSet<Pair<Integer, Integer>> region, char plant,
            Pair<Integer, Integer> position) {
        Integer x = position.getLeft();
        Integer y = position.getRight();

        if (y < 0 || y >= garden.length || x < 0 || x >= garden[y].length || garden[y][x] != plant
                || region.contains(position)) {
            return;
        }

        region.add(position);

        fillRegion(garden, region, plant, Pair.of(x + 1, y));
        fillRegion(garden, region, plant, Pair.of(x - 1, y));
        fillRegion(garden, region, plant, Pair.of(x, y + 1));
        fillRegion(garden, region, plant, Pair.of(x, y - 1));
    }

    private static Integer getRegionArea(HashSet<Pair<Integer, Integer>> region) {
        return region.size();
    }

    private static Integer getRegionPerimeter(HashSet<Pair<Integer, Integer>> region) {
        Integer perimeter = 0;

        for (Pair<Integer, Integer> point : region) {
            Integer x = point.getLeft();
            Integer y = point.getRight();

            if (!region.contains(Pair.of(x + 1, y))) {
                perimeter++;
            }
            if (!region.contains(Pair.of(x - 1, y))) {
                perimeter++;
            }
            if (!region.contains(Pair.of(x, y + 1))) {
                perimeter++;
            }
            if (!region.contains(Pair.of(x, y - 1))) {
                perimeter++;
            }
        }

        return perimeter;
    }

    private static Integer getRegionSides(HashSet<Pair<Integer, Integer>> region) {
        Integer corners = 0;

        for (Pair<Integer, Integer> point : region) {
            Integer x = point.getLeft();
            Integer y = point.getRight();

            boolean hasTopNeighbor = region.contains(Pair.of(x, y - 1));
            boolean hasBottomNeighbor = region.contains(Pair.of(x, y + 1));
            boolean hasLeftNeighbor = region.contains(Pair.of(x - 1, y));
            boolean hasRightNeighbor = region.contains(Pair.of(x + 1, y));

            boolean hasTopLeftNeighbor = region.contains(Pair.of(x - 1, y - 1));
            boolean hasTopRightNeighbor = region.contains(Pair.of(x + 1, y - 1));
            boolean hasBottomLeftNeighbor = region.contains(Pair.of(x - 1, y + 1));
            boolean hasBottomRightNeighbor = region.contains(Pair.of(x + 1, y + 1));

            if (!hasTopNeighbor && !hasLeftNeighbor) {
                corners++;
            }
            if (!hasTopNeighbor && !hasRightNeighbor) {
                corners++;
            }
            if (!hasBottomNeighbor && !hasLeftNeighbor) {
                corners++;
            }
            if (!hasBottomNeighbor && !hasRightNeighbor) {
                corners++;
            }
            if (hasTopNeighbor && hasLeftNeighbor && !hasTopLeftNeighbor) {
                corners++;
            }
            if (hasTopNeighbor && hasRightNeighbor && !hasTopRightNeighbor) {
                corners++;
            }
            if (hasBottomNeighbor && hasLeftNeighbor && !hasBottomLeftNeighbor) {
                corners++;
            }
            if (hasBottomNeighbor && hasRightNeighbor && !hasBottomRightNeighbor) {
                corners++;
            }
        }

        return corners;
    }
}
