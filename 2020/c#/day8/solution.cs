using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Day8 {
    public static void Main() {
        var instructions = readInstructions();

        Console.WriteLine("Part 1: " + part1(instructions));
        Console.WriteLine("Part 2: " + part2(instructions));
    }

    private static int part1(List<string> instructions) {
        return run(instructions).Item1;
    }

    private static int part2(List<string> instructions) {
        for (int index = 0; index < instructions.Count; index++) {
            int acc = 0;
            bool success = false;

            switch (instructions[index].Split(' ')[0]) {
                case "jmp":
                    instructions[index] = instructions[index].Replace("jmp", "nop");
                    (acc, success) = run(instructions);
                    instructions[index] = instructions[index].Replace("nop", "jmp");
                    break;
                case "nop":
                    instructions[index] = instructions[index].Replace("nop", "jmp");
                    (acc, success) = run(instructions);
                    instructions[index] = instructions[index].Replace("jmp", "nop");
                    break;
                default:
                    continue;
            }

            if (success) {
                return acc;
            }
        }
        throw new Exception("no solution found");
    }

    private static List<string> readInstructions() {
        return File.ReadLines(@"input.txt").ToList();
    }

    private static Tuple<int, bool> run(List<string> instructions) {
        int acc = 0;
        int index = 0;
        var executed = new HashSet<int>();

        while (index >= 0 && index < instructions.Count && !executed.Contains(index)) {
            string[] instruction = instructions[index].Split(' ');
            string operation = instruction[0];
            int argument = Int32.Parse(instruction[1]);

            executed.Add(index);
            index++;

            switch (operation) {
                case "acc":
                    acc += argument;
                    break;
                case "jmp":
                    index += argument - 1;
                    break;
            }
        }

        return Tuple.Create(acc, index == instructions.Count);
    }
}
