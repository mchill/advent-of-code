import os
import re

day = os.path.dirname(os.path.realpath(__file__))


valid_moves = {
    "|": [[0, -1], [0, 1]],
    "-": [[1, 0], [-1, 0]],
    "L": [[0, -1], [1, 0]],
    "J": [[0, -1], [-1, 0]],
    "7": [[0, 1], [-1, 0]],
    "F": [[0, 1], [1, 0]],
}


def part1(file: str):
    maze = get_maze(file)
    path = get_path(maze)
    return int(len(path) / 2)


def part2(file: str):
    maze = get_maze(file)
    path = get_path(maze)

    cleaned_maze = ["." * len(row) for row in maze]
    for x, y in path:
        cleaned_maze[y] = replace(cleaned_maze[y], maze[y][x], x)
    maze = cleaned_maze

    enclosed = 0
    for row in maze:
        for x, c in enumerate(row):
            if c == "." and len(re.findall(r"(\||L-*7|F-*J)", row[0:x])) % 2 == 1:
                enclosed += 1
    return enclosed


def get_maze(file: str):
    maze = ["." + line + "." for line in file.splitlines()]
    maze.insert(0, "." * len(maze[0]))
    maze.append("." * len(maze[0]))
    return maze


def get_path(maze: list):
    start = get_start_position(maze)
    maze[start[1]] = replace(maze[start[1]], get_start_pipe(maze, start), start[0])
    path = [start]

    while True:
        cur = path[-1]
        cur_pipe = maze[cur[1]][cur[0]]
        last = path[-2] if len(path) > 1 else None

        next = add(cur, valid_moves[cur_pipe][0])
        if next == last:
            next = add(cur, valid_moves[cur_pipe][1])

        if next == start:
            break
        path.append(next)

    return path


def get_start_position(maze: list):
    for y, row in enumerate(maze):
        for x, cell in enumerate(row):
            if cell == "S":
                return (x, y)


def get_start_pipe(maze: list, start: tuple):
    for pipe in valid_moves:
        for move in valid_moves[pipe]:
            next = add(start, move)
            for next_move in valid_moves.get(maze[next[1]][next[0]], []):
                if move[0] == -next_move[0] and move[1] == -next_move[1]:
                    break
            else:
                break
        else:
            return pipe


def add(p1: (int, int), p2: (int, int)):
    return (p1[0] + p2[0], p1[1] + p2[1])


def replace(string: str, char: str, index: int):
    return string[:index] + char + string[index + 1 :]


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample1.txt") as file:
        content = file.read()
        assert part1(content) == 8

    with open(day + "/sample2.txt") as file:
        content = file.read()
        assert part2(content) == 10
