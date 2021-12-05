using System;
using System.Collections.Generic;

class Day1 {
    public static void Main() {
        var expenses = readExpenses();

        Console.WriteLine("Part 1: " + part1(expenses));
        Console.WriteLine("Part 2: " + part2(expenses));
    }

    private static int part1(List<int> expenses) {
        for (int i = 0; i < expenses.Count; i++) {
            for (int j = i + 1; j < expenses.Count; j++) {
                int expense1 = expenses[i];
                int expense2 = expenses[j];

                if (expense1 + expense2 == 2020) {
                    return expense1 * expense2;
                }
            }
        }
        throw new Exception("no solution found");
    }

    private static int part2(List<int> expenses) {
        for (int i = 0; i < expenses.Count; i++) {
            for (int j = i + 1; j < expenses.Count; j++) {
                for (int k = j + 1; k < expenses.Count; k++) {
                    int expense1 = expenses[i];
                    int expense2 = expenses[j];
                    int expense3 = expenses[k];

                    if (expense1 + expense2 + expense3 == 2020) {
                        return expense1 * expense2 * expense3;
                    }
                }
            }
        }
        throw new Exception("no solution found");
    }

    private static List<int> readExpenses() {
        System.IO.StreamReader file = new System.IO.StreamReader(@"input.txt");
        var expenses = new List<int>();

        string line;
        while ((line = file.ReadLine()) != null) {
            expenses.Add(Int32.Parse(line));
        }

        return expenses;
    }
}
