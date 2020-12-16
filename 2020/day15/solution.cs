using System;
using System.Collections.Generic;
using System.Linq;

class Day15 {
    public static void Main() {
        var numbers = readNumbers();

        Console.WriteLine("Part 1: " + part1(numbers));
        Console.WriteLine("Part 2: " + part2(numbers));
    }

    private static int part1(List<int> numbers) {
        return count(numbers, 2020);
    }

    private static int part2(List<int> numbers) {
        return count(numbers, 30000000);
    }

    private static List<int> readNumbers() {
       return new List<int>(){ 2, 0, 6, 12, 1, 3 };
    }

    private static int count(List<int> numbers, int iterations) {
        Dictionary<int, int> occurrences = numbers.Select((n, i) => new { n, i }).ToDictionary(x => x.n, x => x.i);
        int current = numbers.Last();

        for (int i = numbers.Count; i < iterations; i++) {
            int last = current;
            current = i - 1 - occurrences.GetValueOrDefault(last, i - 1);
            occurrences[last] = i - 1;
        }

        return current;
    }
}
