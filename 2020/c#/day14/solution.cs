using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

class Day14 {
    public static void Main() {
        List<string> instructions = readInstructions();

        Console.WriteLine("Part 1: " + part1(instructions));
        Console.WriteLine("Part 2: " + part2(instructions));
    }

    private static ulong part1(List<string> instructions) {
        var memory = new Dictionary<int, ulong>();
        string mask = "";

        foreach (string instruction in instructions) {
            if (instruction.StartsWith("mask")) {
                mask = instruction.Split(" = ")[1];
                continue;
            }

            int address = Int32.Parse(instruction.Split(new char[] {'[', ']'})[1]);
            ulong value = UInt64.Parse(instruction.Split(" = ")[1]);

            value = value & Convert.ToUInt64(mask.Replace('X', '1'), 2);
            value = value | Convert.ToUInt64(mask.Replace('X', '0'), 2);

            memory[address] = value;
        }

        return memory.Values.Aggregate((acc, value) => acc + value);
    }

    private static ulong part2(List<string> instructions) {
        var memory = new Dictionary<string, ulong>();
        string mask = "";

        foreach (string instruction in instructions) {
            if (instruction.StartsWith("mask")) {
                mask = instruction.Split(" = ")[1];
                continue;
            }

            ulong address = UInt64.Parse(instruction.Split(new char[] {'[', ']'})[1]);
            ulong value = UInt64.Parse(instruction.Split(" = ")[1]);
            address = address | Convert.ToUInt64(mask.Replace('X', '0'), 2);

            List<int> xIndices = mask.Select((ch, index) => ch == 'X' ? index : -1).Where(index => index >= 0).ToList();
            List<string> possibilities = Enumerable.Range(0, Convert.ToInt32(Math.Pow(2, xIndices.Count)))
                .Select(value => Convert.ToString(value, 2).PadLeft(xIndices.Count, '0')).ToList();

            foreach (string possibility in possibilities) {
                StringBuilder tempAddress = new StringBuilder(Convert.ToString((long)address, 2).PadLeft(mask.Length, '0'));
                for (int i = 0; i < xIndices.Count; i++) {
                    tempAddress[xIndices[i]] = possibility[i];
                }
                memory[tempAddress.ToString()] = value;
            }
        }

        return memory.Values.Aggregate((acc, value) => acc + value);
    }

    private static List<string> readInstructions() {
       return File.ReadLines(@"input.txt").ToList();
    }
}
