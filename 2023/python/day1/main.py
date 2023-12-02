import os

day = os.path.dirname(os.path.realpath(__file__))


def main(file, parse_strings=False):
    return sum([find_calibration_value(line, parse_strings) for line in file.splitlines()])


def find_calibration_value(line: str, parse_strings):
    return int(find_first_digit(line, parse_strings) + find_first_digit(line, parse_strings, reverse=True))


def find_first_digit(line: str, parse_strings, reverse=False):
    digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

    for i in range(len(line)) if not reverse else reversed(range(len(line))):
        if line[i].isdigit():
            return line[i]
        if parse_strings:
            for d, digit in enumerate(digits):
                if line[i : i + len(digit)] == digit:
                    return str(d + 1)


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(main(content)))
        print("Part 2: " + str(main(content, parse_strings=True)))


def test():
    with open(day + "/sample1.txt") as file:
        content = file.read()
        assert main(content) == 142

    with open(day + "/sample2.txt") as file:
        content = file.read()
        assert main(content, parse_strings=True) == 281
