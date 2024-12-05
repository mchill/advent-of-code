package io.mchill.day1;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

public class Solution {
    public static void main(String[] args) throws IOException {
        String input = readInput(args[0]);
        System.out.println("Part 1: " + part1(input));
        System.out.println("Part 2: " + part2(input));
    }

    private static String readInput(String path) throws IOException {
        return Files.readString(Paths.get(path));
    }

    private static Integer part1(String input) {
        List<Integer> list1 = new ArrayList<>();
        List<Integer> list2 = new ArrayList<>();

        for (String line : input.split("\n")) {
            List<Integer> coordinates = Arrays.stream(line.split("\\s+"))
                    .map(Integer::parseInt)
                    .collect(Collectors.toList());

            list1.add(coordinates.get(0));
            list2.add(coordinates.get(1));
        }

        list1.sort(Integer::compareTo);
        list2.sort(Integer::compareTo);

        Integer diff = 0;

        for (int i = 0; i < list1.size(); i++) {
            diff += Math.abs(list1.get(i) - list2.get(i));
        }

        return diff;
    }

    private static Integer part2(String input) {
        List<Integer> list1 = new ArrayList<>();
        List<Integer> list2 = new ArrayList<>();

        for (String line : input.split("\n")) {
            List<Integer> coordinates = Arrays.stream(line.split("\\s+"))
                    .map(Integer::parseInt)
                    .collect(Collectors.toList());

            list1.add(coordinates.get(0));
            list2.add(coordinates.get(1));
        }

        Integer similarity = 0;

        for (int number : list1) {
            similarity += number * Collections.frequency(list2, number);
        }

        return similarity;
    }
}
