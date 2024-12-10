package io.mchill.day9;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
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
        List<Integer> blocks = getBlocks(input);

        for (int i = 0; i < blocks.size(); i++) {
            if (blocks.get(i) != null) {
                continue;
            }

            Integer replacement = null;
            while (replacement == null) {
                replacement = blocks.removeLast();
            }

            blocks.set(i, replacement);
        }

        return getChecksum(blocks);
    }

    private static Long part2(String input) {
        List<Integer> blocks = getBlocks(input);

        Integer fileId = null;
        Integer fileLength = 0;

        for (int i = blocks.size() - 1; i >= 0; i--) {
            Integer block = blocks.get(i);

            if (fileId == block) {
                fileLength++;
                continue;
            }

            if (fileId != null) {
                for (int j = 0; j < i + 1; j++) {
                    Boolean canPlace = true;
                    for (int k = 0; k < fileLength; k++) {
                        if (blocks.get(j + k) != null) {
                            canPlace = false;
                            break;
                        }
                    }
                    if (canPlace) {
                        for (int k = 0; k < fileLength; k++) {
                            blocks.set(i + k + 1, null);
                            blocks.set(j + k, fileId);
                        }
                        break;
                    }
                }
            }

            fileId = block;
            fileLength = 1;
        }

        return getChecksum(blocks);
    }

    private static List<Integer> getBlocks(String input) {
        List<Integer> diskMap = input.strip().chars().mapToObj(Character::getNumericValue).collect(Collectors.toList());
        List<Integer> blocks = new ArrayList<>();

        for (int i = 0; i < diskMap.size(); i++) {
            Integer size = diskMap.get(i);
            Integer fileId = i / 2;
            for (int j = 0; j < size; j++) {
                Integer block = i % 2 == 0 ? fileId : null;
                blocks.add(block);
            }
        }

        return blocks;
    }

    private static Long getChecksum(List<Integer> blocks) {
        Long checksum = 0L;
        for (int i = 0; i < blocks.size(); i++) {
            if (blocks.get(i) != null) {
                checksum += i * blocks.get(i);
            }
        }
        return checksum;
    }
}
