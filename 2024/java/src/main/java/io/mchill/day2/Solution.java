package io.mchill.day2;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
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
        Integer safeTotal = 0;

        List<List<Integer>> reports = Arrays.stream(input.split("\n"))
                .map(line -> Arrays.stream(line.split(" ")).map(Integer::valueOf).collect(Collectors.toList()))
                .collect(Collectors.toList());

        for (List<Integer> report : reports) {
            List<List<Integer>> options = Arrays.asList(report, report.reversed());
            safeTotal += options.stream().anyMatch(r -> isReportSafe(r)) ? 1 : 0;
        }

        return safeTotal;
    }

    private static Integer part2(String input) {
        Integer safeTotal = 0;

        List<List<Integer>> reports = Arrays.stream(input.split("\n"))
                .map(line -> Arrays.stream(line.split(" ")).map(Integer::valueOf).collect(Collectors.toList()))
                .collect(Collectors.toList());

        for (List<Integer> report : reports) {
            List<List<Integer>> options = new ArrayList<>();
            options.add(report);
            options.addAll(getDampenedReports(report));
            options.add(report.reversed());
            options.addAll(getDampenedReports(report.reversed()));
            safeTotal += options.stream().anyMatch(r -> isReportSafe(r)) ? 1 : 0;
        }

        return safeTotal;
    }

    private static Boolean isReportSafe(List<Integer> report) {
        for (int i = 1; i < report.size(); i++) {
            if (report.get(i) <= report.get(i - 1) || report.get(i) > report.get(i - 1) + 3) {
                return false;
            }
        }
        return true;
    }

    private static List<List<Integer>> getDampenedReports(List<Integer> report) {
        List<List<Integer>> reports = new ArrayList<List<Integer>>();
        for (int i = 0; i < report.size(); i++) {
            List<Integer> dampenedReport = new ArrayList<>(report);
            dampenedReport.remove(i);
            reports.add(dampenedReport);
        }
        return reports;
    }
}
