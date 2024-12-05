package io.mchill.day3;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Solution {
    public static void main(String[] args) throws IOException {
        String input = readInput(args[0]).replace("\n", "");
        System.out.println("Part 1: " + part1(input));
        System.out.println("Part 2: " + part2(input));
    }

    private static String readInput(String path) throws IOException {
        return Files.readString(Paths.get(path));
    }

    private static Integer part1(String input) {
        return execute(input);
    }

    private static Integer part2(String input) {
        Pattern pattern = Pattern.compile("do\\(\\).*?don't\\(\\)");
        Matcher matcher = pattern.matcher("do()" + input + "don't()");

        Integer total = 0;

        while (matcher.find()) {
            total += execute(matcher.group());
        }

        return total;
    }

    private static Integer execute(String input) {
        Pattern pattern = Pattern.compile("mul\\(\\d{1,3},\\d{1,3}\\)");
        Matcher matcher = pattern.matcher(input);

        Integer total = 0;

        while (matcher.find()) {
            String expression = matcher.group();
            List<Integer> numbers = List.of(expression.substring(4, expression.length() - 1).split(",")).stream()
                    .map(Integer::parseInt).collect(Collectors.toList());
            total += numbers.get(0) * numbers.get(1);
        }

        return total;
    }
}
