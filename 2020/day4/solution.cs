using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

public class Passport {
    private Dictionary<string, string> passport = new Dictionary<string, string>();

    public Passport(string passport) {
        foreach (string field in passport.Split(new char[]{' ', '\n'}, StringSplitOptions.RemoveEmptyEntries)) {
            this.passport.Add(field.Split(':')[0], field.Split(':')[1]);
        }
    }

    public bool isComplete() {
        return new List<string>(){ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" }.All(field => passport.ContainsKey(field));
    }

    public bool isValid() {
        if (!isComplete()) {
            return false;
        }

        string hgt = passport["hgt"];
        int hgtMeasurement = Int32.Parse(hgt.TrimEnd("cmin".ToCharArray()));
        string hgtUnit = hgt.Substring(hgt.Length - 2);

        return between(Int32.Parse(passport["byr"]), 1920, 2002) &&
            between(Int32.Parse(passport["iyr"]), 2010, 2020) &&
            between(Int32.Parse(passport["eyr"]), 2020, 2030) && (
                hgtUnit == "cm" && hgtMeasurement >= 150 && hgtMeasurement <= 193 ||
                hgtUnit == "in" && hgtMeasurement >= 59 && hgtMeasurement <= 76
            ) &&
            new Regex(@"^#[a-f0-9]{6}$").IsMatch(passport["hcl"]) &&
            new List<string>() { "amb", "blu", "brn", "gry", "grn", "hzl", "oth" }.Contains(passport["ecl"]) &&
            passport["pid"].Length == 9;
    }

    public bool between(int value, int lower, int upper) {
        return value >= lower && value <= upper;
    }
}

class Day4 {
    public static void Main() {
        var passports = readPassports();

        Console.WriteLine("Part 1: " + part1(passports));
        Console.WriteLine("Part 2: " + part2(passports));
    }

    private static long part1(IEnumerable<Passport> passports) {
        return passports.Count(passport => passport.isComplete());
    }

    private static long part2(IEnumerable<Passport> passports) {
        return passports.Count(passport => passport.isValid());
    }

    private static IEnumerable<Passport> readPassports() {
        return File.ReadAllText(@"input.txt").Split("\n\n").ToList().Select(passport => new Passport(passport));
    }
}
