package io.mchill.day24;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map.Entry;
import java.util.TreeMap;

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
        String[] sections = input.split("\n\n");

        TreeMap<String, Boolean> wires = new TreeMap<>();
        List<String> instructions = new ArrayList<>(List.of(sections[1].split("\n")));

        for (String line : sections[0].split("\n")) {
            String[] parts = line.split(": ");
            wires.put(parts[0], parts[1].equals("1"));
        }

        while (instructions.size() > 0) {
            for (int i = 0; i < instructions.size(); i++) {
                String[] parts = instructions.get(i).split(" ");
                String input1 = parts[0];
                String operator = parts[1];
                String input2 = parts[2];
                String output = parts[4];

                if (!wires.containsKey(input1) || !wires.containsKey(input2)) {
                    continue;
                }

                switch (operator) {
                    case "AND":
                        wires.put(output, wires.get(input1) && wires.get(input2));
                        break;
                    case "OR":
                        wires.put(output, wires.get(input1) || wires.get(input2));
                        break;
                    case "XOR":
                        wires.put(output, wires.get(input1) ^ wires.get(input2));
                        break;
                    default:
                        throw new IllegalArgumentException("Invalid operator: " + operator);
                }

                instructions.remove(i);
                i--;
            }
        }

        StringBuilder output = new StringBuilder();
        wires.entrySet().stream().filter(entry -> entry.getKey().startsWith("z")).forEach(entry -> {
            output.append(entry.getValue() ? "1" : "0");
        });

        return Long.parseLong(output.reverse().toString(), 2);
    }

    private static String part2(String input) {
        String[] sections = input.split("\n\n");
        HashMap<String, String> instructions = new HashMap<>();
        HashSet<String> swappedWires = new HashSet<>();

        for (String line : sections[1].split("\n")) {
            String[] parts = line.split(" -> ");
            instructions.put(parts[1], parts[0]);
        }

        for (Entry<String, String> entry : instructions.entrySet()) {
            String wire = entry.getKey();
            String instruction = entry.getValue();

            String[] parts = instruction.split(" ");
            String operator = parts[1];

            String input1 = parts[0];
            String input2 = parts[2];

            String input1Instruction = instructions.get(input1);
            String input2Instruction = instructions.get(input2);

            Boolean childOfXAndY = input1.startsWith("x") && input2.startsWith("y")
                    || input1.startsWith("y") && input2.startsWith("x");

            if (wire.startsWith("z") && !wire.equals("z45")) {
                if (!instruction.contains("XOR")) {
                    swappedWires.add(wire);
                }
            }
            switch (operator) {
                case "XOR":
                    if (wire.startsWith("z") && childOfXAndY || !wire.startsWith("z") && !childOfXAndY) {
                        swappedWires.add(wire);
                    }
                case "AND":
                    if (childOfXAndY) {
                        continue;
                    }
                    if (input1Instruction.contains("AND")) {
                        swappedWires.add(input1);
                    }
                    if (input2Instruction.contains("AND")) {
                        swappedWires.add(input2);
                    }
                    break;
                case "OR":
                    if (!input1Instruction.contains("AND")) {
                        swappedWires.add(input1);
                    }
                    if (!input2Instruction.contains("AND")) {
                        swappedWires.add(input2);
                    }
                    break;
                default:
                    throw new IllegalArgumentException("Invalid operator: " + operator);
            }
        }

        // Remove outliers at beginning and end of graph
        swappedWires.remove("nvd");
        swappedWires.remove("z00");
        swappedWires.remove("z45");

        return String.join(",", swappedWires.stream().sorted().toList());
    }
}
