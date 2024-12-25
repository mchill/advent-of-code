package io.mchill.day25;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Solution {
    public static void main(String[] args) throws IOException {
        String input = readInput(args[0]);
        System.out.println("Part 1: " + part1(input));
    }

    private static String readInput(String path) throws IOException {
        String packagePath = Solution.class.getPackageName().replace('.', '/');
        String fullPath = Paths.get("src/main/java", packagePath, path).toString();
        return Files.readString(Paths.get(fullPath));
    }

    private static Integer part1(String input) {
        List<List<Integer>> locks = new ArrayList<>();
        List<List<Integer>> keys = new ArrayList<>();

        for (String section : input.split("\n\n")) {
            String[] schematic = section.split("\n");
            List<Integer> lockOrKey = new ArrayList<>();

            for (int column = 0; column < 5; column++) {
                Integer count = 0;
                for (String line : schematic) {
                    if (line.charAt(column) == '#') {
                        count++;
                    }
                }
                lockOrKey.add(count);
            }

            if (schematic[0].equals("#####")) {
                locks.add(lockOrKey);
            } else {
                keys.add(lockOrKey);
            }
        }

        Integer matches = 0;

        for (List<Integer> key : keys) {
            for (List<Integer> lock : locks) {
                Boolean fits = true;
                for (int i = 0; i < 5; i++) {
                    if (key.get(i) + lock.get(i) > 7) {
                        fits = false;
                        break;
                    }
                }
                if (fits) {
                    matches++;
                }
            }
        }

        return matches;
    }
}
