package io.mchill.day8;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
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
        Pair<Integer, Integer> bounds = getBounds(input);
        HashMap<Character, List<Pair<Integer, Integer>>> antennas = getAntennas(input);
        HashSet<Pair<Integer, Integer>> antiNodes = getAntiNodes(antennas, bounds, false);

        return antiNodes.size();
    }

    private static Integer part2(List<String> input) {
        Pair<Integer, Integer> bounds = getBounds(input);
        HashMap<Character, List<Pair<Integer, Integer>>> antennas = getAntennas(input);
        HashSet<Pair<Integer, Integer>> antiNodes = getAntiNodes(antennas, bounds, true);

        return antiNodes.size();
    }

    private static Pair<Integer, Integer> getBounds(List<String> input) {
        return Pair.of(input.get(0).length(), input.size());
    }

    private static HashMap<Character, List<Pair<Integer, Integer>>> getAntennas(List<String> input) {
        HashMap<Character, List<Pair<Integer, Integer>>> antennas = new HashMap<>();

        Integer rows = input.size();
        Integer columns = input.get(0).length();

        for (int y = 0; y < rows; y++) {
            for (int x = 0; x < columns; x++) {
                char c = input.get(y).charAt(x);
                if (c == '.') {
                    continue;
                }
                antennas.putIfAbsent(c, new ArrayList<>());
                antennas.get(c).add(Pair.of(x, y));
            }
        }

        return antennas;
    }

    private static HashSet<Pair<Integer, Integer>> getAntiNodes(
            HashMap<Character, List<Pair<Integer, Integer>>> antennas, Pair<Integer, Integer> bounds,
            Boolean resonance) {
        HashSet<Pair<Integer, Integer>> antiNodes = new HashSet<>();

        for (Character c : antennas.keySet()) {
            for (int i = 0; i < antennas.get(c).size(); i++) {
                for (int j = i + 1; j < antennas.get(c).size(); j++) {
                    Pair<Integer, Integer> antenna1 = antennas.get(c).get(i);
                    Pair<Integer, Integer> antenna2 = antennas.get(c).get(j);

                    if (antenna1.equals(antenna2)) {
                        continue;
                    }

                    if (resonance) {
                        antiNodes.add(antenna1);
                        antiNodes.add(antenna2);
                    }

                    Pair<Integer, Integer> diff = subtractPair(antenna1, antenna2);
                    Pair<Integer, Integer> antiNode = addPair(antenna1, diff);

                    while (inBounds(antiNode, bounds)) {
                        antiNodes.add(antiNode);
                        antiNode = addPair(antiNode, diff);

                        if (!resonance) {
                            break;
                        }
                    }

                    antiNode = subtractPair(antenna2, diff);

                    while (inBounds(antiNode, bounds)) {
                        antiNodes.add(antiNode);
                        antiNode = subtractPair(antiNode, diff);

                        if (!resonance) {
                            break;
                        }
                    }
                }
            }
        }
        return antiNodes;
    }

    private static Pair<Integer, Integer> addPair(Pair<Integer, Integer> p1, Pair<Integer, Integer> p2) {
        return Pair.of(p1.getLeft() + p2.getLeft(), p1.getRight() + p2.getRight());
    }

    private static Pair<Integer, Integer> subtractPair(Pair<Integer, Integer> p1, Pair<Integer, Integer> p2) {
        return Pair.of(p1.getLeft() - p2.getLeft(), p1.getRight() - p2.getRight());
    }

    private static Boolean inBounds(Pair<Integer, Integer> p, Pair<Integer, Integer> bounds) {
        return p.getLeft() >= 0 && p.getLeft() < bounds.getLeft() && p.getRight() >= 0
                && p.getRight() < bounds.getRight();
    }
}
