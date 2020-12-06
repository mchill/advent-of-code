using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Day5 {
    public static void Main() {
        var seats = readBoardingPasses();

        Console.WriteLine("Part 1: " + part1(seats));
        Console.WriteLine("Part 2: " + part2(seats));
    }

    private static long part1(IEnumerable<int> seats) {
        return seats.Max();
    }

    private static long part2(IEnumerable<int> seats) {
        var allSeats = new HashSet<int>(Enumerable.Range(0, 127 * 8 + 7)).Except(seats);
        for (int seat = 1; seat < allSeats.Count() - 1; seat++) {
            int current = allSeats.ElementAt(seat);
            if (current > allSeats.ElementAt(seat - 1) + 1 && current < allSeats.ElementAt(seat + 1) - 1) {
                return current;
            }
        }
        throw new Exception("no solution found");
    }

    private static IEnumerable<int> readBoardingPasses() {
        return File.ReadLines(@"input.txt").Select(seat => {
            return Convert.ToInt32(seat.Replace('F', '0').Replace('B', '1').Replace('L', '0').Replace('R', '1'), 2);
        });
    }
}
