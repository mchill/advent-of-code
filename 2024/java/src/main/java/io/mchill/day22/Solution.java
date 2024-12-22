package io.mchill.day22;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

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

    private static Long part1(List<String> input) {
        Long sum = 0L;

        for (String line : input) {
            Long secret = Long.parseLong(line);
            for (int i = 0; i < 2000; i++) {
                secret = prune(mix(secret, secret * 64));
                secret = prune(mix(secret, secret / 32));
                secret = prune(mix(secret, secret * 2048));
            }
            sum += secret;
        }

        return sum;
    }

    private static Long part2(List<String> input) {
        List<HashMap<String, Long>> sequencesList = new ArrayList<>();

        for (String line : input) {
            Long secret = Long.parseLong(line);

            Queue<Long> changes = new LinkedList<>();
            HashMap<String, Long> sequences = new HashMap<>();
            sequencesList.add(sequences);

            for (int i = 0; i < 2000; i++) {
                Long oldSecret = secret;

                secret = prune(mix(secret, secret * 64));
                secret = prune(mix(secret, secret / 32));
                secret = prune(mix(secret, secret * 2048));

                changes.add(secret % 10 - oldSecret % 10);
                if (changes.size() > 4) {
                    changes.remove();
                }

                if (changes.size() == 4) {
                    String sequence = String.join(",", changes.stream().map(Object::toString).toArray(String[]::new));
                    sequences.putIfAbsent(sequence, secret % 10);
                }
            }
        }

        HashSet<String> allSequences = new HashSet<>();
        for (HashMap<String, Long> sequence : sequencesList) {
            allSequences.addAll(sequence.keySet());
        }

        Long max = 0L;
        for (String sequence : allSequences) {
            Long bananas = 0L;
            for (HashMap<String, Long> sequences : sequencesList) {
                bananas += sequences.getOrDefault(sequence, 0L);
            }
            max = Math.max(max, bananas);
        }
        return max;
    }

    private static Long mix(Long secret, Long value) {
        return secret ^ value;
    }

    private static Long prune(Long secret) {
        return secret % 16777216;
    }
}
