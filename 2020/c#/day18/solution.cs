using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

class Day18 {
    public static void Main() {
        var expressions = readExpressions();

        Console.WriteLine("Part 1: " + part1(expressions));
        Console.WriteLine("Part 2: " + part2(expressions));
    }

    private static long part1(List<string> expressions) {
        return expressions.Select(expression => solve(expression, simpleSolver)).Sum();
    }

    private static long part2(List<string> expressions) {
        return expressions.Select(expression => solve(expression, advancedSolver)).Sum();
    }

    private static long solve(string expression, Func<string, long> solver) {
        Regex subExpression = new Regex(@"\(([^()]+)\)");
        while (expression.Contains('(')) {
            expression = subExpression.Replace(expression, match => solver(match.Groups[1].Value).ToString());
        }
        return solver(expression);
    }

    private static long simpleSolver(string rawExpression) {
        var operatorMap = new Dictionary<string, Func<long, long, long>>(){
            {"+", (x, y) => x + y},
            {"*", (x, y) => x * y}
        };
        List<string> expression = rawExpression.Split().Prepend("+").ToList();
        long solution = 0;
        for (int i = 1; i < expression.Count; i += 2) {
            solution = operatorMap[expression[i - 1]](solution, Int64.Parse(expression[i]));
        }
        return solution;
    }

    private static long advancedSolver(string expression) {
        return expression.Split('*').Aggregate(1L, (acc, part) => acc *= part.Split('+').Select(number => Int64.Parse(number)).Sum());
    }

    private static List<string> readExpressions() {
        return File.ReadLines(@"input.txt").ToList();
    }
}
