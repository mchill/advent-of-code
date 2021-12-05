using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Space {
    private int dimensions;
    private HashSet<string> space = new HashSet<string>();

    public Space(string[] initial, int dimensions) {
        this.dimensions = dimensions;

        for (int y = 0; y < initial.Length; y++) {
            for (int x = 0; x < initial[y].Length; x++) {
                if (initial[y][x] == '#') {
                    space.Add($"{x},{y}{string.Concat(Enumerable.Repeat(",0", dimensions - 2))}");
                }
            }
        }
    }

    public int GetActive() {
        return space.Count;
    }

    public void RunCycle() {
        var newSpace = new HashSet<string>(space);

        Dictionary<string, int> activeNeighborsMap = new Dictionary<string, int>();
        foreach (string coordinates in space) {
            foreach (string neighbor in getNeighbors(coordinates)) {
                activeNeighborsMap[neighbor] = activeNeighborsMap.GetValueOrDefault(neighbor, 0) + 1;
            }
        }

        foreach(string coordinates in activeNeighborsMap.Keys) {
            bool active = space.Contains(coordinates);
            int activeNeighbors = activeNeighborsMap[coordinates] - (active ? 1 : 0);

            if (active && activeNeighbors != 2 && activeNeighbors != 3) {
                newSpace.Remove(coordinates);
            } else if (!active && activeNeighbors == 3) {
                newSpace.Add(coordinates);
            }
        }

        space = newSpace;
    }

    private List<string> getNeighbors(string coordinates) {
        List<Tuple<int, int>> bounds = coordinates.Split(',').Select(c => Int32.Parse(c)).Select(c => Tuple.Create(c - 1, c + 1)).ToList();
        return getPointsInBounds(0, bounds);
    }

    private List<string> getPointsInBounds(int dimension, List<Tuple<int, int>> bounds) {
        Tuple<int, int> range = bounds[dimension];
        List<string> coordinates = Enumerable.Range(range.Item1, range.Item2 - range.Item1 + 1).Select(c => c.ToString()).ToList();
        if (dimension == dimensions - 1) {
            return coordinates;
        }
        return coordinates.Select(c => getPointsInBounds(dimension + 1, bounds).Select(cs => $"{c},{cs}")).SelectMany(c => c).ToList();
    }
}

class Day17 {
    public static void Main() {
        var space = readSpace();

        Console.WriteLine("Part 1: " + runCycles(new Space(space, 3)));
        Console.WriteLine("Part 2: " + runCycles(new Space(space, 4)));
    }

    private static int runCycles(Space space) {
        foreach (var _ in Enumerable.Range(0, 6)) {
            space.RunCycle();
        }
        return space.GetActive();
    }

    private static string[] readSpace() {
        return File.ReadAllLines(@"input.txt");
    }
}
