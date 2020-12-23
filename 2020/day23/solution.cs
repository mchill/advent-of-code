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
        var cups = readCups();

        Console.WriteLine("Part 1: " + part1(new LinkedList<int>(cups)));
        Console.WriteLine("Part 2: " + part2(new LinkedList<int>(cups)));
    }

    private static string part1(LinkedList<int> cups) {
        game(cups, 100);
        while (cups.First.Value != 1) {
            var last = cups.Last;
            cups.RemoveLast();
            cups.AddFirst(last);
        }
        return string.Join("", cups.Skip(1));
    }

    private static long part2(LinkedList<int> cups) {
        foreach (int cup in Enumerable.Range(cups.Count + 1, 1000000 - cups.Count)) {
            cups.AddLast(cup);
        }
        game(cups, 10000000);
        var cup1 = cups.Find(1);
        return Convert.ToInt64(cup1.NextOrFirst().Value) * cup1.NextOrFirst().NextOrFirst().Value;
    }

    private static void game(LinkedList<int> cups, int moves) {
        var cupMap = new Dictionary<int, LinkedListNode<int>>();
        LinkedListNode<int> current = cups.First;
        while (current != null) {
            cupMap[current.Value] = current;
            current = current.Next;
        }

        current = cups.First;
        int cupCount = cups.Count;
        var removed = new Stack<LinkedListNode<int>>();

        foreach (var _ in Enumerable.Range(0, moves)) {
            while (removed.Count < 3) {
                removed.Push(current.NextOrFirst());
                cups.Remove(current.NextOrFirst());
            }
            LinkedListNode<int> destination = null;
            for (int i = cupCount - 2; i >= 0; i--) {
                destination = cupMap[(current.Value + i) % cupCount + 1];
                if (!removed.Contains(destination)) {
                    break;
                }
            }
            while (removed.Count > 0) {
                cups.AddAfter(destination, removed.Pop());
            }
            current = current.NextOrFirst();
        }
    }

    private static LinkedList<int> readCups() {
        return new LinkedList<int>("418976235".Select(c => Int32.Parse(c.ToString())));
    }
}
