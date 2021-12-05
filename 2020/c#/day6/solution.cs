using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Day6 {
    public static void Main() {
        var groups = readAnswers();

        Console.WriteLine("Part 1: " + part1(groups));
        Console.WriteLine("Part 2: " + part2(groups));
    }

    private static int part1(IEnumerable<string[]> groups) {
        return groups.Sum(group => group.Skip(1).Aggregate(new HashSet<char>(group.First()), unionWith).Count());
    }

    private static int part2(IEnumerable<string[]> groups) {
        return groups.Sum(group => group.Skip(1).Aggregate(new HashSet<char>(group.First()), intersectWith).Count());
    }

    private static IEnumerable<string[]> readAnswers() {
        return File.ReadAllText(@"input.txt").Split("\n\n").ToList().Select(group => group.Split('\n', StringSplitOptions.RemoveEmptyEntries));
    }

    private static HashSet<char> unionWith(HashSet<char> set, string person) {
        set.UnionWith(person);
        return set;
    }

    private static HashSet<char> intersectWith(HashSet<char> set, string person) {
        set.IntersectWith(person);
        return set;
    }
}
