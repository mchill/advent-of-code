package io.mchill.day23;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;

public class Solution {
    private static HashMap<List<String>, List<String>> cache = new HashMap<>();

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
        HashMap<String, List<String>> connections = getConnections(input);
        HashSet<String> groups = new HashSet<>();

        for (String c1 : connections.keySet()) {
            for (String c2 : connections.get(c1)) {
                for (String c3 : connections.get(c2)) {
                    if (connections.get(c3).contains(c1)
                            && (c1.startsWith("t") || c2.startsWith("t") || c3.startsWith("t"))) {
                        groups.add(String.join(",", List.of(c1, c2, c3).stream().sorted().toList()));
                    }
                }
            }
        }

        return groups.stream().filter(group -> group.contains("t")).count();
    }

    private static String part2(List<String> input) {
        HashMap<String, List<String>> connections = getConnections(input);

        List<String> largestGroup = new ArrayList<>();

        for (String c : connections.keySet()) {
            List<String> group = getLargestGroup(connections, List.of(c));
            if (group.size() > largestGroup.size()) {
                largestGroup = group;
            }
        }

        return String.join(",", largestGroup);
    }

    private static HashMap<String, List<String>> getConnections(List<String> input) {
        HashMap<String, List<String>> connections = new HashMap<>();

        for (String line : input) {
            String[] parts = line.split("-");
            String from = parts[0];
            String to = parts[1];
            connections.putIfAbsent(from, new ArrayList<>());
            connections.get(from).add(to);
            connections.putIfAbsent(to, new ArrayList<>());
            connections.get(to).add(from);
        }

        return connections;
    }

    private static List<String> getLargestGroup(HashMap<String, List<String>> connections, List<String> group) {
        if (cache.containsKey(group)) {
            return cache.get(group);
        }

        List<String> sharedConnections = new ArrayList<>(connections.get(group.get(0)));
        for (String c : group) {
            sharedConnections.retainAll(connections.get(c));
        }
        sharedConnections.removeAll(group);

        if (sharedConnections.isEmpty()) {
            cache.put(group, group);
            return group;
        }

        List<String> largestGroup = new ArrayList<>();

        for (String c : sharedConnections) {
            List<String> newGroup = new ArrayList<>(group);
            newGroup.add(c);
            newGroup = getLargestGroup(connections, newGroup.stream().sorted().toList());

            if (newGroup.size() > largestGroup.size()) {
                largestGroup = newGroup;
            }
        }

        cache.put(group, largestGroup);
        return largestGroup;
    }
}
