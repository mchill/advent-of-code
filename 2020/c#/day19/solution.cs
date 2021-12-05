using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

class Day19 {
    public static void Main() {
        (var rules, var messages) = readInput();

        Console.WriteLine("Part 1: " + part1(rules, messages));
        Console.WriteLine("Part 2: " + part2(rules, messages));
    }

    private static int part1(Dictionary<string, List<List<string>>> rules, List<string> messages) {
        Regex regex = new Regex($"^{buildRegex(rules, "0")}$");
        return messages.Count(message => regex.IsMatch(message));
    }

    private static int part2(Dictionary<string, List<List<string>>> rules, List<string> messages) {
        string subrule31 = buildRegex(rules, "31");
        string subrule42 = buildRegex(rules, "42");
        string subrule8 = $"{subrule42}+";
        string subrule11 = $"(?<c>{subrule42})+(?<-c>{subrule31})+(?(c)(?!))";
        string subrule0 = $"^{subrule8}{subrule11}$";

        Regex regex = new Regex(subrule0);
        return messages.Count(message => regex.IsMatch(message));
    }

    private static Tuple<Dictionary<string, List<List<string>>>, List<string>> readInput() {
        List<List<string>> file = File.ReadAllText(@"input.txt").Split("\n\n").Select(chunk => chunk.Split('\n').ToList()).ToList();
        Dictionary<string, List<List<string>>> rules = file[0].Select(line => line.Split(": ")).ToDictionary(
            line => line[0],
            line => line[1].Split(" | ").Select(subrule => subrule.Split().ToList()).ToList()
        );
        return Tuple.Create(rules, file[1]);
    }

    private static string buildRegex(Dictionary<string, List<List<string>>> rules, string currentRule) {
        if (!rules.ContainsKey(currentRule)) {
            return currentRule.Trim('"');
        }
        string regex = string.Join('|', rules[currentRule].Select(rule => string.Join("", rule.Select(subrule => buildRegex(rules, subrule)))));
        return $"({regex})";
    }
}
