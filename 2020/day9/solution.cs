using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Day8 {
    public const int Preamble = 25;

    public static void Main() {
        var numbers = readNumbers();
        long invalid = part1(numbers);

        Console.WriteLine("Part 1: " + invalid);
        Console.WriteLine("Part 2: " + part2(numbers, invalid));
    }

    private static long part1(List<long> numbers) {
        var sums = new Queue<long>();

        for (int i = 0; i < numbers.Count; i++) {
            if (i > Preamble && !sums.Contains(numbers[i])) {
                return numbers[i];
            }
            for (int k = 0; k < Math.Min(Preamble, i - Preamble - 1); k++) {
                sums.Dequeue();
            }
            for (int k = Math.Max(0, i - Preamble); k < i; k++) {
                sums.Enqueue(numbers[i] + numbers[k]);
            }
        }

        throw new Exception("no solution found");
    }

    private static long part2(List<long> numbers, long invalid) {
        long sum = numbers[0] + numbers[1];
        int start = 0;
        int end = 1;

        while (start < end && end < numbers.Count) {
            if (sum < invalid) {
                end++;
                sum += numbers[end];
            } else if (sum == invalid) {
                IEnumerable<long> subset = numbers.Skip(start).Take(end - start);
                return subset.Min() + subset.Max();
            } else if (sum > invalid) {
                sum -= numbers[start];
                start++;
            }
        }

        throw new Exception("no solution found");
    }

    private static List<long> readNumbers() {
        return File.ReadLines(@"input.txt").Select(line => Int64.Parse(line)).ToList();
    }
}
