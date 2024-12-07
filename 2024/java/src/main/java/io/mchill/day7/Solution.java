package io.mchill.day7;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

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
        Long total = 0L;

        for (String line : input) {
            String[] parts = line.split(": ");
            Long test = Long.parseLong(parts[0]);
            List<Long> numbers = Arrays.stream(parts[1].split(" ")).map(Long::parseLong).collect(Collectors.toList());

            if (evaluate(test, numbers, numbers.removeFirst(), false)) {
                total += test;
            }
        }

        return total;
    }

    private static Long part2(List<String> input) {
        Long total = 0L;

        for (String line : input) {
            String[] parts = line.split(": ");
            Long test = Long.parseLong(parts[0]);
            List<Long> numbers = Arrays.stream(parts[1].split(" ")).map(Long::parseLong).collect(Collectors.toList());

            if (evaluate(test, numbers, numbers.removeFirst(), true)) {
                total += test;
            }
        }

        return total;
    }

    private static Boolean evaluate(Long test, List<Long> numbers, Long result, Boolean includeConcatenateRule) {
        if (numbers.size() == 0) {
            return test.equals(result);
        }

        List<Long> newNumbers = new ArrayList<>(numbers);
        Long first = newNumbers.removeFirst();

        return evaluate(test, new ArrayList<>(newNumbers), result + first, includeConcatenateRule)
                || evaluate(test, new ArrayList<>(newNumbers), result * first, includeConcatenateRule)
                || (includeConcatenateRule && evaluate(test, new ArrayList<>(newNumbers),
                        Long.parseLong(result.toString() + first), includeConcatenateRule));
    }
}
