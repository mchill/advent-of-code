using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

public class PasswordEntry {
    public string Password;
    public char Character;
    public int Lower;
    public int Upper;
}

class Day2 {
    public static void Main() {
        var passwords = readPasswords();

        Console.WriteLine("Part 1: " + part1(passwords));
        Console.WriteLine("Part 2: " + part2(passwords));
    }

    private static int part1(List<PasswordEntry> passwords) {
        return passwords.Count(password => {
            int count = password.Password.Count(c => c == password.Character);
            return count >= password.Lower && count <= password.Upper;
        });
    }

    private static int part2(List<PasswordEntry> passwords) {
        return passwords.Count(password => {
            return password.Password[password.Lower - 1] == password.Character ^
                password.Password[password.Upper - 1] == password.Character;
        });
    }

    private static List<PasswordEntry> readPasswords() {
        System.IO.StreamReader file = new System.IO.StreamReader(@"input.txt");
        var passwords = new List<PasswordEntry>();

        string line;
        while ((line = file.ReadLine()) != null) {
            var parts = line.Split(new char[] { '-', ' ', ':' }, StringSplitOptions.RemoveEmptyEntries);
            var password = new PasswordEntry {
                Lower = Int32.Parse(parts[0]),
                Upper = Int32.Parse(parts[1]),
                Character = parts[2][0],
                Password = parts[3]
            };
            passwords.Add(password);
        }

        return passwords;
    }
}
