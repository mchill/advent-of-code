using System;
using System.IO;

class Day3 {
    public static void Main() {
        var trees = readTrees();

        Console.WriteLine("Part 1: " + part1(trees));
        Console.WriteLine("Part 2: " + part2(trees));
    }

    private static long part1(string[] trees) {
        return checkSlope(trees, 3, 1);
    }

    private static long part2(string[] trees) {
        return checkSlope(trees, 1, 1) *
            checkSlope(trees, 3, 1) *
            checkSlope(trees, 5, 1) *
            checkSlope(trees, 7, 1) *
            checkSlope(trees, 1, 2);
    }

    private static long checkSlope(string[] trees, int right, int down) {
        long encountered = 0;
        for (int row = 0; row < trees.Length; row += down) {
            if (trees[row][(row / down * right) % trees[row].Length] == '#') {
                encountered++;
            }
        }
        return encountered;
    }

    private static string[] readTrees() {
        return File.ReadAllLines(@"input.txt");
    }
}
