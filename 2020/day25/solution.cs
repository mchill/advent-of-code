using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Day25 {
    public static void Main() {
        (long cardKey, long doorKey) = readPublicKeys();

        Console.WriteLine("Part 1: " + part1(cardKey, doorKey));
    }

    private static long part1(long cardKey, long doorKey) {
        int loopSize = 0;

        for (long value = 1; value != cardKey; loopSize++) {
            value = (value * 7) % 20201227;
        }

        long encryptionKey = 1;
        foreach (var _ in Enumerable.Range(0, loopSize)) {
            encryptionKey = (encryptionKey * doorKey) % 20201227;
        }

        return encryptionKey;
    }

    private static Tuple<long, long> readPublicKeys() {
        List<long> keys = File.ReadLines(@"input.txt").Select(line => Int64.Parse(line)).ToList();
        return Tuple.Create(keys[0], keys[1]);
    }
}
