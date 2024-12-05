import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

class Solution {
    public static void main(String[] args) throws IOException {
        List<String> input = readInput(args[0]);
        System.out.println("Part 1: " + part1(input));
        System.out.println("Part 2: " + part2(input));
    }

    private static List<String> readInput(String path) throws IOException {
        return Files.readAllLines(Paths.get(path));
    }

    private static Integer part1(List<String> input) {
        char[][] crossword = pad2DList(input).stream().map(String::toCharArray).toArray(char[][]::new);

        Integer matches = 0;

        for (int i = 1; i < crossword.length - 1; i++) {
            for (int j = 1; j < crossword[i].length - 1; j++) {
                matches += (isXmasInDirection(crossword, i, j, 1, 0) ? 1 : 0)
                        + (isXmasInDirection(crossword, i, j, 1, 1) ? 1 : 0)
                        + (isXmasInDirection(crossword, i, j, 0, 1) ? 1 : 0)
                        + (isXmasInDirection(crossword, i, j, -1, 1) ? 1 : 0)
                        + (isXmasInDirection(crossword, i, j, -1, 0) ? 1 : 0)
                        + (isXmasInDirection(crossword, i, j, -1, -1) ? 1 : 0)
                        + (isXmasInDirection(crossword, i, j, 0, -1) ? 1 : 0)
                        + (isXmasInDirection(crossword, i, j, 1, -1) ? 1 : 0);
            }
        }

        return matches;
    }

    private static Integer part2(List<String> input) {
        char[][] crossword = pad2DList(input).stream().map(String::toCharArray).toArray(char[][]::new);

        Integer matches = 0;

        for (int i = 1; i < crossword.length - 1; i++) {
            for (int j = 1; j < crossword[i].length - 1; j++) {
                matches += isXmas(crossword, i, j) ? 1 : 0;
            }
        }

        return matches;
    }

    private static List<String> pad2DList(List<String> input) {
        List<String> lines = new ArrayList<String>(input);
        String padding = ".".repeat(lines.get(0).length());
        lines.add(0, padding);
        lines.add(padding);
        return lines.stream().map(line -> ("." + line + ".")).toList();
    }

    private static Boolean isXmasInDirection(char[][] crossword, int i, int j, int x, int y) {
        return crossword[i][j] == 'X' && crossword[i + x][j + y] == 'M'
                && crossword[i + 2 * x][j + 2 * y] == 'A' && crossword[i + 3 * x][j + 3 * y] == 'S';
    }

    private static Boolean isXmas(char[][] crossword, int i, int j) {
        return crossword[i][j] == 'A'
                && (crossword[i - 1][j - 1] == 'M' && crossword[i + 1][j + 1] == 'S'
                        || crossword[i - 1][j - 1] == 'S' && crossword[i + 1][j + 1] == 'M')
                && (crossword[i + 1][j - 1] == 'M' && crossword[i - 1][j + 1] == 'S'
                        || crossword[i + 1][j - 1] == 'S' && crossword[i - 1][j + 1] == 'M');
    }
}
