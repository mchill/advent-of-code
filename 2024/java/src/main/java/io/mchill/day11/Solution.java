package io.mchill.day11;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.HashMap;
import java.util.stream.Collectors;

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

    private static Long part1(String input) {
        HashMap<Long, Long> stones = Arrays.stream(input.strip().split(" "))
                .map(Long::parseLong)
                .collect(Collectors.toMap(stone -> stone, stone -> 1L, Long::sum, HashMap::new));

        for (int i = 0; i < 25; i++) {
            stones = blink(stones);
        }

        return stones.values().stream().mapToLong(Long::longValue).sum();
    }

    private static Long part2(String input) {
        HashMap<Long, Long> stones = Arrays.stream(input.strip().split(" "))
                .map(Long::parseLong)
                .collect(Collectors.toMap(stone -> stone, stone -> 1L, Long::sum, HashMap::new));

        for (int i = 0; i < 75; i++) {
            stones = blink(stones);
        }

        return stones.values().stream().mapToLong(Long::longValue).sum();
    }

    private static HashMap<Long, Long> blink(HashMap<Long, Long> stones) {
        HashMap<Long, Long> newStones = new HashMap<>();

        for (Long stone : stones.keySet()) {
            Long count = stones.get(stone);
            String stoneStr = stone.toString();

            if (stone == 0) {
                newStones.put(1L, newStones.getOrDefault(1L, 0L) + count);
            } else if (stoneStr.length() % 2 == 0) {
                long newStone1 = Long.parseLong(stoneStr.substring(0, stoneStr.length() / 2));
                long newStone2 = Long.parseLong(stoneStr.substring(stoneStr.length() / 2, stoneStr.length()));
                newStones.put(newStone1, newStones.getOrDefault(newStone1, 0L) + count);
                newStones.put(newStone2, newStones.getOrDefault(newStone2, 0L) + count);
            } else {
                newStones.put(stone * 2024, newStones.getOrDefault(stone * 2024, 0L) + count);
            }
        }

        return newStones;
    }
}
