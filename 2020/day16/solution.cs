using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class RuleSet {
    public string name;
    private List<Rule> rules;

    public RuleSet(string line) {
        List<string> parts = line.Split(": ").ToList();
        name = parts[0];
        rules = parts[1].Split(" or ").Select(rule => new Rule(rule)).ToList();
    }

    public bool IsValid(int value) {
        return rules.Any(rule => rule.IsValid(value));
    }
}

class Rule {
    private int min;
    private int max;

    public Rule(string rule) {
        List<int> range = rule.Split('-').Select(number => Int32.Parse(number)).ToList();
        min = range[0];
        max = range[1];
    }

    public bool IsValid(int value) {
        return value >= min && value <= max;
    }
}

class Day16 {
    public static void Main() {
        (List<RuleSet> rules, List<int> myTicket, List<List<int>> tickets) = readInput();

        Console.WriteLine("Part 1: " + part1(rules, tickets));
        Console.WriteLine("Part 2: " + part2(rules, myTicket, tickets));
    }

    private static int part1(List<RuleSet> rules, List<List<int>> tickets) {
        return tickets.SelectMany(value => value).Where(value => rules.All(rule => !rule.IsValid(value))).Sum();
    }

    private static long part2(List<RuleSet> rules, List<int> myTicket, List<List<int>> tickets) {
        List<List<int>> validTickets = tickets.Where(values => values.All(value => rules.Any(rule => rule.IsValid(value)))).ToList();

        Dictionary<RuleSet, List<int>> ruleOptions = rules.ToDictionary(
            rule => rule,
            rule => Enumerable.Range(0, rules.Count).Where(position => validTickets.All(ticket => rule.IsValid(ticket[position]))).ToList()
        );

        List<KeyValuePair<RuleSet, List<int>>> sortedRuleOptions = ruleOptions.OrderBy(rule => rule.Value.Count).ToList();
        for (int i = 0; i < sortedRuleOptions.Count; i++) {
            for (int j = i + 1; j < sortedRuleOptions.Count; j++) {
                sortedRuleOptions[j].Value.Remove(sortedRuleOptions[i].Value[0]);
            }
        }

        return rules.Where(rule => rule.name.StartsWith("departure")).Aggregate(1L, (acc, rule) => acc * myTicket[ruleOptions[rule][0]]);
    }

    private static Tuple<List<RuleSet>, List<int>, List<List<int>>> readInput() {
        List<List<string>> chunks = File.ReadAllText(@"input.txt").Split("\n\n").Select(chunk => chunk.Split('\n', StringSplitOptions.RemoveEmptyEntries).ToList()).ToList();

        List<RuleSet> rules = chunks[0].Select(line => new RuleSet(line)).ToList();
        List<int> myTicket = chunks[1][1].Split(',').Select(value => Int32.Parse(value)).ToList();
        List<List<int>> tickets = chunks[2].Skip(1).Select(line => line.Split(',').Select(value => Int32.Parse(value)).ToList()).ToList();

        return Tuple.Create(rules, myTicket, tickets);
    }
}
