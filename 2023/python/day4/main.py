import os

day = os.path.dirname(os.path.realpath(__file__))


def main(file: str):
    part1 = 0
    cards = {card: 1 for card in range(len(file.splitlines()))}
    lines = [[[num for num in nums.split()] for nums in line.split(": ")[1].split(" | ")] for line in file.splitlines()]

    for card, [winning, numbers] in enumerate(lines):
        matches = len(set(numbers).intersection(set(winning)))
        if matches > 0:
            part1 += 2 ** (matches - 1)
        for i in range(matches):
            cards[card + i + 1] += cards[card]

    return part1, sum(cards.values())


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        part1, part2 = main(content)
        print("Part 1: " + str(part1))
        print("Part 2: " + str(part2))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        part1, part2 = main(content)
        assert part1 == 13
        assert part2 == 30
