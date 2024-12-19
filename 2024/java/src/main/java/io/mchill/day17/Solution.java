package io.mchill.day17;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;

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

    private static String part1(List<String> input) {
        Long a = Long.parseLong(input.get(0).split(": ")[1]);
        List<Long> out = runProgram(a);
        return String.join(",", out.stream().map(String::valueOf).toList());
    }

    private static Long part2(List<String> input) {
        List<Long> program = Arrays.stream(input.get(4).split(": ")[1].split(",")).map(Long::parseLong).toList();
        List<Long> inputs = runReversedProgram(program, 0L, program.size() - 1);
        return inputs.stream().min(Comparator.naturalOrder()).orElse(null);
    }

    private static List<Long> runProgram(Long a) {
        List<Long> out = new ArrayList<>();
        while (a > 0) {
            out.add(runStep(a));
            a = a / 8;
        }
        return out;
    }

    private static List<Long> runReversedProgram(List<Long> program, Long a, Integer index) {
        List<Long> inputs = new ArrayList<>();
        for (long nextA = a * 8; nextA < a * 8 + 8; nextA++) {
            if (runStep(nextA) != program.get(index)) {
                continue;
            }
            if (index == 0) {
                inputs.add(nextA);
                continue;
            }
            inputs.addAll(runReversedProgram(program, nextA, index - 1));
        }
        return inputs;
    }

    private static Long runStep(Long a) {
        Long b = a % 8;
        b = b ^ 2;
        Long c = a / (1L << b);
        b = b ^ 3;
        b = b ^ c;
        return b % 8;
    }
}
