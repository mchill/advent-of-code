using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

public struct Bag {
    public string color;
    public Dictionary<string, int> children;
    public List<string> parents;

    public Bag(string bag) {
        color = string.Join(" ", bag.Split(' ').Take(2));
        parents = new List<string>();
        children = new Dictionary<string, int>();
        if (!bag.EndsWith("no other bags.")) {
            children = bag.TrimEnd('.')
                .Split(',')
                .Select(child => child.Split(' ').TakeLast(4))
                .ToDictionary(bag => string.Join(" ", bag.Skip(1).Take(2)), bag => Int32.Parse(bag.First()));
        }
    }
}

class Day7 {
    public static void Main() {
        var bags = readBags();

        Console.WriteLine("Part 1: " + part1(bags));
        Console.WriteLine("Part 2: " + part2(bags));
    }

    private static int part1(Dictionary<string, Bag> bags) {
        return getAncestors(bags["shiny gold"], bags).Count();
    }

    private static int part2(Dictionary<string, Bag> bags) {
        return getDescendants(bags["shiny gold"], bags);
    }

    private static Dictionary<string, Bag> readBags() {
        Dictionary<string, Bag> bags = File.ReadLines(@"input.txt").Select(bag => new Bag(bag)).ToDictionary(bag => bag.color, bag => bag);
        foreach (Bag bag in bags.Values) {
            foreach (string child in bag.children.Keys) {
                bags[child].parents.Add(bag.color);
            }
        }
        return bags;
    }

    private static List<string> getAncestors(Bag bag, Dictionary<string, Bag> bags) {
        return bag.parents.Aggregate(bag.parents, (acc, parent) => acc.Union(getAncestors(bags[parent], bags)).ToList());
    }

    private static int getDescendants(Bag bag, Dictionary<string, Bag> bags) {
        return bag.children.Aggregate(0, (acc, child) => acc + (1 + getDescendants(bags[child.Key], bags)) * child.Value);
    }
}
