using System;
using System.Collections.Generic;
using System.IO;

class Vector {
    public Tuple<int, int> vector;

    public Vector(int x, int y) {
        vector = Tuple.Create(x, y);
    }

    public void move(Vector direction, int distance) {
        vector = Tuple.Create(vector.Item1 + direction.vector.Item1 * distance, vector.Item2 + direction.vector.Item2 * distance);
    }

    public void rotate(int degrees) {
        double radians = degrees * Math.PI / 180;
        vector = Tuple.Create(
            Convert.ToInt32(vector.Item1 * Math.Cos(radians) - vector.Item2 * Math.Sin(radians)),
            Convert.ToInt32(vector.Item1 * Math.Sin(radians) + vector.Item2 * Math.Cos(radians))
        );
    }

    public int manhattanDistance() {
        return Math.Abs(vector.Item1) + Math.Abs(vector.Item2);
    }
}

class Day12 {
    private static Dictionary<char, Vector> directionMap = new Dictionary<char, Vector>(){
        {'E', new Vector(1, 0)},
        {'S', new Vector(0, 1)},
        {'W', new Vector(-1, 0)},
        {'N', new Vector(0, -1)}
    };

    public static void Main() {
        var instructions = readInstructions();

        Console.WriteLine("Part 1: " + part1(instructions));
        Console.WriteLine("Part 2: " + part2(instructions));
    }

    private static int part1(string[] instructions) {
        return sail(instructions, new Vector(1, 0), true);
    }

    private static int part2(string[] instructions) {
        return sail(instructions, new Vector(10, -1), false);
    }

    private static int sail(string[] instructions, Vector waypoint, bool fixedWaypoint) {
        Vector position = new Vector(0, 0);

        foreach (string instruction in instructions) {
            char action = instruction[0];
            int value = Int32.Parse(instruction.Substring(1));

            switch (action) {
                case 'L':
                    waypoint.rotate(-value);
                    break;
                case 'R':
                    waypoint.rotate(value);
                    break;
                case 'F':
                    position.move(waypoint, value);
                    break;
                default:
                    if (fixedWaypoint) {
                        position.move(directionMap[action], value);
                    } else {
                        waypoint.move(directionMap[action], value);
                    }
                    break;
            }
        }

        return position.manhattanDistance();
    }

    private static string[] readInstructions() {
        return File.ReadAllLines(@"input.txt");
    }
}
