package io.mchill.day5;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.regex.Pattern;

import org.apache.commons.lang3.tuple.Pair;

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
        var rulesAndUpdates = buildRulesAndUpdates(input);
        HashMap<String, List<String>> rules = rulesAndUpdates.getLeft();
        List<List<String>> updates = rulesAndUpdates.getRight();

        Integer total = 0;

        for (List<String> pages : updates) {
            List<String> sortedPages = new ArrayList<>(pages);
            sortedPages.sort((a, b) -> comparePages(a, b, rules));

            if (pages.equals(sortedPages)) {
                total += Integer.parseInt(pages.get(pages.size() / 2));
            }
        }

        return total;
    }

    private static Integer part2(String input) {
        var rulesAndUpdates = buildRulesAndUpdates(input);
        HashMap<String, List<String>> rules = rulesAndUpdates.getLeft();
        List<List<String>> updates = rulesAndUpdates.getRight();

        Integer total = 0;

        for (List<String> pages : updates) {
            List<String> sortedPages = new ArrayList<>(pages);
            sortedPages.sort((a, b) -> comparePages(a, b, rules));

            if (!pages.equals(sortedPages)) {
                total += Integer.parseInt(pages.get(pages.size() / 2));
            }
        }

        return total;
    }

    private static Pair<HashMap<String, List<String>>, List<List<String>>> buildRulesAndUpdates(String input) {
        String[] sections = input.split("\n\n");
        String[] ruleSection = sections[0].split("\n");
        String[] updateSection = sections[1].split("\n");

        HashMap<String, List<String>> rules = new HashMap<>();
        List<List<String>> updates = new ArrayList<>();

        for (String rule : ruleSection) {
            String[] pages = rule.split(Pattern.quote("|"));
            rules.putIfAbsent(pages[0], new ArrayList<>());
            rules.get(pages[0]).add(pages[1]);
        }

        for (int i = 0; i < updateSection.length; i++) {
            String[] pages = updateSection[i].split(Pattern.quote(","));
            updates.add(Arrays.asList(pages));
        }

        return Pair.of(rules, updates);
    }

    private static int comparePages(String a, String b, HashMap<String, List<String>> rules) {
        return rules.getOrDefault(a, new ArrayList<>()).contains(b) ? -1 : 1;
    }
}
