using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

public class Image {
    public int size;
    public int tileSize;
    public Tile[,] spaces;
    private int currentSpace = 0;

    public Image(int size, int tileSize) {
        this.size = size;
        this.tileSize = tileSize;
        spaces = new Tile[size, size];
    }

    public bool Place(Tile tile) {
        int x = currentSpace % size;
        int y = currentSpace / size;

        Tile left = x > 0 ? spaces[y, x - 1] : null;
        Tile up = y > 0 ? spaces[y - 1, x] : null;

        if ((left == null || tile.left == left.right) && (up == null || tile.top == up.bottom)) {
            spaces[y, x] = tile;
            currentSpace++;
            return true;
        }

        return false;
    }

    public void Remove() {
        currentSpace--;
        spaces[currentSpace / size, currentSpace % size] = null;
    }

    public bool IsComplete() {
        return currentSpace == size * size;
    }

    public void Rotate() {
        var rotated = new Tile[size, size];
        for (int x = 0; x < size; x++) {
            for (int y = 0; y < size; y++) {
                spaces[y, x].Rotate();
                rotated[x, y] = spaces[size - y - 1, x];
            }
        }
        spaces = rotated;
    }

    public void Flip() {
        var flipped = new Tile[size, size];
        for (int x = 0; x < size; x++) {
            for (int y = 0; y < size; y++) {
                spaces[y, x].Flip();
                flipped[y, x] = spaces[y, size - x - 1];
            }
        }
        spaces = flipped;
    }

    public override string ToString() {
        var image = new StringBuilder();
        for (int y = 0; y < size; y++) {
            for (int row = 0; row < tileSize - 2; row++) {
                for (int x = 0; x < size; x++) {
                    image.Append(spaces[y, x].ToString().Split('\n')[row]);
                }
                image.Append('\n');
            }
        }
        return image.ToString();
    }
}

public class Tile {
    public long id;
    public string right, bottom, left, top;
    public int size;
    private char[,] tile;

    public Tile(int id, List<string> lines) {
        size = lines.Count;
        tile = new char[size, size];
        this.id = id;
        for (int x = 0; x < size; x++) {
            for (int y = 0; y < size; y++) {
                tile[x, y] = lines[x][y];
            }
        }
        setSides();
    }

    public void Rotate() {
        var rotated = new char[size, size];
        for (int x = 0; x < size; x++) {
            for (int y = 0; y < size; y++) {
                rotated[x, y] = tile[size - y - 1, x];
            }
        }
        tile = rotated;
        setSides();
    }

    public void Flip() {
        var flipped = new char[size, size];
        for (int x = 0; x < size; x++) {
            for (int y = 0; y < size; y++) {
                flipped[y, x] = tile[y, size - x - 1];
            }
        }
        tile = flipped;
        setSides();
    }

    public override string ToString() {
        return string.Join('\n', Enumerable.Range(1, size - 2).Select(y => string.Join("", Enumerable.Range(1, size - 2).Select(x => tile[y, x]))));
    }

    private void setSides() {
        right = string.Join("", Enumerable.Range(0, size).Select(i => tile[i, size - 1]));
        bottom = string.Join("", Enumerable.Range(0, size).Select(i => tile[size - 1, i]));
        left = string.Join("", Enumerable.Range(0, size).Select(i => tile[i, 0]));
        top = string.Join("", Enumerable.Range(0, size).Select(i => tile[0, i]));
    }
}

class Day20 {
    public static void Main() {
        var tiles = readTiles();
        Image image = new Image(Convert.ToInt32(Math.Sqrt(tiles.Count)), tiles[0].size);
        place(image, new Queue<Tile>(tiles));

        Console.WriteLine("Part 1: " + part1(image));
        Console.WriteLine("Part 2: " + part2(image));
    }

    private static long part1(Image image) {
        return image.spaces[0, 0].id *
            image.spaces[0, image.size - 1].id *
            image.spaces[image.size - 1, 0].id *
            image.spaces[image.size - 1, image.size - 1].id;
    }

    private static int part2(Image image) {
        int width = image.size * (image.tileSize - 2) - 18;
        Regex monster = new Regex($"(?<=#.{{{width}}})#....##....##....###(?=.{{{width}}}#..#..#..#..#..#)", RegexOptions.Singleline);

        foreach (var _flip in Enumerable.Range(0, 2)) {
            foreach (var _rotate in Enumerable.Range(0, 4)) {
                string imageStr = image.ToString();
                int monsters = monster.Matches(imageStr).Count;
                if (monsters > 0) {
                    return imageStr.Count(c => c == '#') - monsters * 15;
                }
                image.Rotate();
            }
            image.Flip();
        }
        throw new Exception("no sea monsters found");
    }

    private static List<Tile> readTiles() {
        return File.ReadAllText(@"input.txt")
            .Split("\n\n")
            .Select(tile => tile.Split('\n', StringSplitOptions.RemoveEmptyEntries))
            .Select(tile => new Tile(Int32.Parse(tile.First().Substring(5, 4)), tile.Skip(1).ToList()))
            .ToList();
    }

    private static void place(Image image, Queue<Tile> tiles) {
        foreach (var _ in Enumerable.Range(0, tiles.Count)) {
            Tile tile = tiles.Dequeue();
            foreach (var _flip in Enumerable.Range(0, 2)) {
                foreach (var _rotate in Enumerable.Range(0, 4)) {
                    if (image.Place(tile)) {
                        place(image, new Queue<Tile>(tiles));
                        if (image.IsComplete()) {
                            return;
                        }
                        image.Remove();
                    }
                    tile.Rotate();
                }
                tile.Flip();
            }
            tiles.Enqueue(tile);
        }
    }
}
