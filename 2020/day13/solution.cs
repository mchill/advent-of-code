using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Day13 {
    public static void Main() {
        (int timestamp, List<int> buses) = readBuses();

        Console.WriteLine("Part 1: " + part1(timestamp, buses));
        Console.WriteLine("Part 2: " + part2(buses));
    }

    private static int part1(int timestamp, List<int> buses) {
        List<int> delays = buses.Select(bus => bus - timestamp % bus).ToList();
        int smallestDelay = delays.Where(delay => delay > 1).Min();
        return smallestDelay * buses[delays.IndexOf(smallestDelay)];
    }

    private static long part2(List<int> buses) {
        long timestamp = 0;
        long lcm = 1;

        for (int synced = 0; synced < buses.Count; timestamp += lcm) {
            for (int index = synced; index < buses.Count; index++) {
                int bus = buses[index];
                if ((bus - timestamp - index) % bus != 0) {
                    break;
                }
                synced++;
                lcm = getLCM(lcm, bus);
            }
        }

        return timestamp - lcm;
    }

    private static Tuple<int, List<int>> readBuses() {
        List<string> lines = File.ReadLines(@"input.txt").ToList();
        int timestamp = Int32.Parse(lines.First());
        List<int> buses = lines.Last().Replace('x', '1').Split(',').Select(bus => Int32.Parse(bus)).ToList();
        return Tuple.Create(timestamp, buses);
    }

    private static long getLCM(long a, long b) {
        return (a / getGCF(a, b)) * b;
    }

    private static long getGCF(long a, long b) {
        while (b != 0) {
            long temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }
}
