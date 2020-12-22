using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Day22 {
    public static void Main() {
        var players = readDecks();

        Console.WriteLine("Part 1: " + part1(players.Select(player => new Queue<int>(player)).ToList()));
        Console.WriteLine("Part 2: " + part2(players.Select(player => new Queue<int>(player)).ToList()));
    }

    private static long part1(List<Queue<int>> players) {
        return players[combat(players, false)].Reverse().Select((card, index) => card * (index + 1)).Sum();
    }

    private static int part2(List<Queue<int>> players) {
        return players[combat(players, true)].Reverse().Select((card, index) => card * (index + 1)).Sum();
    }

    private static List<Queue<int>> readDecks() {
        return File.ReadAllText(@"input.txt")
            .Split("\n\n")
            .Select(chunk => new Queue<int>(chunk.Split('\n', StringSplitOptions.RemoveEmptyEntries).Skip(1).Select(line => Int32.Parse(line))))
            .ToList();
    }

    private static int combat(List<Queue<int>> players, bool isRecursive) {
        var encountered = new HashSet<string>();

        while (players.All(player => player.Count > 0)) {
            if (!encountered.Add(string.Join(' ', players.Select(player => string.Join(',', player))))) {
                return 0;
            }

            var winner = players.OrderByDescending(player => player.Peek()).First();
            if (isRecursive && players.All((player) => player.Count > player.Peek())) {
                winner = players[combat(players.Select(player => new Queue<int>(player.Skip(1).Take(player.Peek()))).ToList(), isRecursive)];
            }

            winner.Enqueue(winner.Dequeue());
            players.Where(player => player != winner).ToList().ForEach(player => winner.Enqueue(player.Dequeue()));
        }

        return players.FindIndex(player => player.Count > 0);
    }
}
