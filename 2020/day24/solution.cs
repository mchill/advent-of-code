using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

class Day24 {
    public static void Main() {
        var directions = readDirections();
        var floor = new HashSet<Tuple<int, int>>();

        Console.WriteLine("Part 1: " + part1(directions, floor));
        Console.WriteLine("Part 2: " + part2(floor));
    }

    private static long part1(List<string> directions, HashSet<Tuple<int, int>> floor) {
        foreach (string direction in directions) {
            int se = Regex.Matches(direction, "se").Count;
            int ne = Regex.Matches(direction, "ne").Count;
            int e = Regex.Matches(direction, "e").Count - se - ne;
            int sw = Regex.Matches(direction, "sw").Count;
            int nw = Regex.Matches(direction, "nw").Count;
            int w = Regex.Matches(direction, "w").Count - sw - nw;

            int axis1 = se + e - (nw + w);
            int axis2 = ne + e - (sw + w);

            var position = Tuple.Create(axis1, axis2);
            if (!floor.Add(position)) {
                floor.Remove(position);
            }
        }

        return floor.Count;
    }

    private static int part2(HashSet<Tuple<int, int>> floor) {
        var adjacentOffsets = new List<Tuple<int, int>>() {
            Tuple.Create(-1, -1),
            Tuple.Create(-1, 0),
            Tuple.Create(0, -1),
            Tuple.Create(0, 0),
            Tuple.Create(0, 1),
            Tuple.Create(1, 0),
            Tuple.Create(1, 1),
        };

        foreach(var _ in Enumerable.Range(0, 100)) {
            var adjacencies = new Dictionary<Tuple<int, int>, int>();

            foreach (var position in floor) {
                adjacentOffsets.ForEach(offset => {
                    var adjacent = Tuple.Create(position.Item1 + offset.Item1, position.Item2 + offset.Item2);
                    adjacencies.TryAdd(adjacent, 0);
                    adjacencies[adjacent]++;
                });
            }

            foreach (var adjacent in adjacencies) {
                if (floor.Contains(adjacent.Key) && (adjacent.Value == 1 || adjacent.Value > 3)) {
                    floor.Remove(adjacent.Key);
                } else if (!floor.Contains(adjacent.Key) && adjacent.Value == 2) {
                    floor.Add(adjacent.Key);
                }
            }
        }

        return floor.Count;
    }

    private static List<string> readDirections() {
        return File.ReadLines(@"input.txt").ToList();
    }
}
