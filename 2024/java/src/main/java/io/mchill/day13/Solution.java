package io.mchill.day13;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

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
        List<SystemOfEquations> systems = getSystemsOfEquations(input);
        Long total = 0L;

        for (SystemOfEquations system : systems) {
            total += system.solve();
        }

        return total;
    }

    private static Long part2(String input) {
        List<SystemOfEquations> systems = getSystemsOfEquations(input);
        Long total = 0L;

        for (SystemOfEquations system : systems) {
            system.e += 10000000000000L;
            system.f += 10000000000000L;
            total += system.solve();
        }

        return total;
    }

    private static List<SystemOfEquations> getSystemsOfEquations(String input) {
        Pattern pattern = Pattern.compile("\\d+");
        String[] sections = input.split("\n\n");

        List<SystemOfEquations> systems = new ArrayList<>();

        for (String section : sections) {
            String[] lines = section.split("\n");

            Matcher matcher = pattern.matcher(lines[0]);
            matcher.find();
            Long a = Long.parseLong(matcher.group());
            matcher.find();
            Long c = Long.parseLong(matcher.group());

            matcher = pattern.matcher(lines[1]);
            matcher.find();
            Long b = Long.parseLong(matcher.group());
            matcher.find();
            Long d = Long.parseLong(matcher.group());

            matcher = pattern.matcher(lines[2]);
            matcher.find();
            Long e = Long.parseLong(matcher.group());
            matcher.find();
            Long f = Long.parseLong(matcher.group());

            SystemOfEquations equation = new SystemOfEquations(a, b, c, d, e, f);
            systems.add(equation);
        }

        return systems;
    }

    static private class SystemOfEquations {
        Long a, b, c, d, e, f;

        public SystemOfEquations(Long a, Long b, Long c, Long d, Long e, Long f) {
            this.a = a;
            this.b = b;
            this.c = c;
            this.d = d;
            this.e = e;
            this.f = f;
        }

        /*
         * Solves a system of two linear equations in the form of:
         * ax + by = e
         * cx + dy = f
         */
        public Long solve() {
            double det = (a * d - b * c);
            double x = (d * e - b * f) / det;
            double y = (a * f - c * e) / det;
            return (x % 1 != 0 || y % 1 != 0) ? 0L : (long) (x * 3 + y);
        }
    }
}
