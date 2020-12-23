using System;
using System.Collections.Generic;
using System.Linq;

static class CircularLinkedList {
    public static LinkedListNode<T> NextOrFirst<T>(this LinkedListNode<T> current) {
        return current.Next ?? current.List.First;
    }
}

class Day23 {
    public static void Main() {
        (var cups, var first, var last) = readCups();

        Console.WriteLine("Part 1: " + part1((int[])cups.Clone(), first));
        Console.WriteLine("Part 2: " + part2((int[])cups.Clone(), first, last));
    }

    private static string part1(int[] cups, int first) {
        game(cups, first, 100);

        string result = "";
        int current = cups[0];
        while (current != 0) {
            result += (current + 1);
            current = cups[current];
        }

        return string.Join("", result);
    }

    private static long part2(int[] cups, int first, int last) {
        int cupCount = cups.Length;
        cups[last] = cupCount;
        Array.Resize(ref cups, 1000000);
        foreach (int cup in Enumerable.Range(cupCount, 1000000 - cupCount)) {
            cups[cup] = cup + 1;
        }
        cups[cups.Length - 1] = first;

        game(cups, first, 10000000);
        return Convert.ToInt64(cups[0] + 1) * (cups[cups[0]] + 1);
    }

    private static void game(int[] cups, int current, int moves) {
        foreach (var _ in Enumerable.Range(0, moves)) {
            int removed = cups[current];
            cups[current] = cups[cups[cups[cups[current]]]];

            int destination = (current + cups.Length - 1) % cups.Length;
            while (destination == removed || destination == cups[removed] || destination == cups[cups[removed]]) {
                destination = (destination + cups.Length - 1) % cups.Length;
            }

            cups[cups[cups[removed]]] = cups[destination];
            cups[destination] = removed;

            current = cups[current];
        }
    }

    private static Tuple<int[], int, int> readCups() {
        List<int> cups = "418976235".Select(c => Int32.Parse(c.ToString()) - 1).ToList();
        return Tuple.Create(
            Enumerable.Range(0, cups.Count).Select(i => cups[(cups.IndexOf(i) + 1) % cups.Count]).ToArray(),
            cups.First(),
            cups.Last()
        );
    }
}
