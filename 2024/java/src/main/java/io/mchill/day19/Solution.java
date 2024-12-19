package io.mchill.day19;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;

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
        String[] towels = parts[0].split(", ");
        String[] designs = parts[1].split("\n");

        HashMap<String, Long> cache = new HashMap<>();
        Integer possible = 0;

        for (String design : designs) {
            if (addTowel(design, towels, cache) > 0) {
                possible++;
            }
        }

        return possible;
    }

    private static Long part2(String input) {
        String[] parts = input.split("\n\n");
        String[] towels = parts[0].split(", ");
        String[] designs = parts[1].split("\n");

        HashMap<String, Long> cache = new HashMap<>();
        Long possible = 0L;

        for (String design : designs) {
            possible += addTowel(design, towels, cache);
        }

        return possible;
    }

    private static Long addTowel(String design, String[] towels, HashMap<String, Long> cache) {
        if (cache.containsKey(design)) {
            return cache.get(design);
        }
        if (design.isEmpty()) {
            return 1L;
        }

        Long possible = 0L;
        for (String towel : towels) {
            if (design.startsWith(towel)) {
                possible += addTowel(design.substring(towel.length()), towels, cache);
            }
        }

        cache.put(design, possible);
        return possible;
    }
}
