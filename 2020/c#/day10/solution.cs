using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Day10 {
    public static void Main() {
        var adapters = readAdapters();

        Console.WriteLine("Part 1: " + part1(adapters));
        Console.WriteLine("Part 2: " + part2(adapters));
    }

    private static int part1(List<int> adapters) {
        var diffs = new Dictionary<int, int>();

        for (int i = 1; i < adapters.Count; i++) {
            int diff = adapters[i] - adapters[i - 1];
            diffs.TryAdd(diff, 0);
            diffs[diff]++;
        }

        return diffs[1] * diffs[3];
    }

    private static long part2(List<int> adapters) {
        var solutions = new Dictionary<int, long>(){{adapters.Count - 1, 1}};

        for (int i = adapters.Count - 2; i >= 0; i--) {
            solutions.Add(i, 0);
            for (int k = i + 1; k <= i + 3 && k < adapters.Count; k++) {
                if (adapters[k] - adapters[i] <= 3) {
                    solutions[i] += solutions[k];
                }
            }
        }

        return solutions[0];
    }

    private static List<int> readAdapters() {
        List<int> adapters = File.ReadLines(@"input.txt").Select(line => Int32.Parse(line)).ToList();
        adapters.Add(0);
        adapters.Sort();
        adapters.Add(adapters.Last() + 3);
        return adapters;
    }
}
