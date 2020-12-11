using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

enum Seat { Floor, Empty, Occupied }

class Day11 {
    public static void Main() {
        var seats = readSeats();

        Console.WriteLine("Part 1: " + part1(seats));
        Console.WriteLine("Part 2: " + part2(seats));
    }

    private static int part1(List<List<Seat>> seats) {
        return simulate(seats, 4, 1);
    }

    private static int part2(List<List<Seat>> seats) {
        return simulate(seats, 5, Math.Max(seats.Count, seats[0].Count));
    }

    private static List<List<Seat>> readSeats() {
        var seatMap = new Dictionary<char, Seat>(){
            {'.', Seat.Floor},
            {'L', Seat.Empty},
            {'#', Seat.Occupied}
        };
        return File.ReadLines(@"input.txt").Select(line => line.Select(seat => seatMap[seat]).ToList()).ToList();
    }

    private static int simulate(List<List<Seat>> seats, int occupiedLimit, int visibleRange) {
        bool changed = false;

        do {
            changed = false;
            var next = seats.Select(row => new List<Seat>(row)).ToList();

            for (int row = 0; row < seats.Count; row++) {
                for (int column = 0; column < seats[0].Count; column++) {
                    switch (seats[row][column]) {
                        case Seat.Empty:
                            if (checkVisiblyAdjacent(seats, row, column, visibleRange) == 0) {
                                next[row][column] = Seat.Occupied;
                                changed = true;
                            }
                            break;
                        case Seat.Occupied:
                            if (checkVisiblyAdjacent(seats, row, column, visibleRange) >= occupiedLimit) {
                                next[row][column] = Seat.Empty;
                                changed = true;
                            }
                            break;
                    }
                }
            }

            seats = next;
        } while (changed);

        return seats.Sum(row => row.Count(seat => seat == Seat.Occupied));
    }

    private static int checkVisiblyAdjacent(List<List<Seat>> seats, int targetRow, int targetColumn, int range) {
        int occupied = 0;

        for (int vertical = -1; vertical <= 1; vertical++) {
            for (int horizontal = -1; horizontal <= 1; horizontal++) {
                if (vertical == 0 && horizontal == 0) {
                    continue;
                }

                for (int step = 1; step <= range; step++) {
                    int row = targetRow + vertical * step;
                    int column = targetColumn + horizontal * step;

                    if (row < 0 || row >= seats.Count || column < 0 || column >= seats[0].Count || seats[row][column] == Seat.Empty) {
                        break;
                    } else if (seats[row][column] == Seat.Occupied) {
                        occupied++;
                        break;
                    }
                }
            }
        }

        return occupied;
    }
}
