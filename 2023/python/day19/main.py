import os
import re
from dataclasses import dataclass
from functools import reduce

day = os.path.dirname(os.path.realpath(__file__))


@dataclass
class Rule:
    workflow: str
    destination: str
    compare: str = None
    operator: str = None
    value: int = None


def part1(file: str):
    part1 = 0
    accepted_ranges = get_accepted_ranges(get_workflows(file))
    for line in file.split("\n\n")[1].splitlines():
        (x, m, a, s) = re.match(r"{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}", line).groups()
        part = {"x": int(x), "m": int(m), "a": int(a), "s": int(s)}
        for r in accepted_ranges:
            if all(r[k][0] <= part[k] <= r[k][1] for k in part.keys()):
                part1 += sum(part.values())
                break
    return part1


def part2(file: str):
    return sum(
        reduce(lambda value, r: value * (r[1] - r[0] + 1), ranges.values(), 1)
        for ranges in get_accepted_ranges(get_workflows(file))
    )


def get_workflows(file):
    workflows = {}
    for line in file.split("\n\n")[0].splitlines():
        (name, rules) = re.match(r"([a-z]+){(.+)}", line).groups()
        workflows[name] = []
        for rule in rules.split(","):
            if ":" not in rule:
                workflows[name].append(Rule(name, rule))
                continue
            (compare, operator, value, destination) = re.match(r"([a-z]+)(<|>)(\d+):([a-zA-Z]+)", rule).groups()
            workflows[name].append(Rule(name, destination, compare, operator, int(value)))
    return workflows


def get_accepted_ranges(workflows):
    all_ranges = []
    for rules in workflows.values():
        for rule in rules:
            if rule.destination == "A":
                upstream_workflow, upstream_ranges = find_upstream(workflows, find_rule=rule)
                ranges = upstream_ranges
                while upstream_workflow != "in":
                    upstream_workflow, upstream_ranges = find_upstream(workflows, find_name=upstream_workflow)
                    ranges = {k: get_overlap(ranges[k], upstream_ranges[k]) for k in ranges.keys()}
                all_ranges.append(ranges)
    return all_ranges


def find_upstream(workflows, find_rule=None, find_name=None):
    for workflow, rules in workflows.items():
        ranges = {"x": [1, 4000], "m": [1, 4000], "a": [1, 4000], "s": [1, 4000]}
        for rule in rules:
            if rule == find_rule or rule.destination == find_name:
                if rule.value is not None:
                    ranges[rule.compare] = get_overlap(
                        ranges[rule.compare], [1, rule.value - 1] if rule.operator == "<" else [rule.value + 1, 4000]
                    )
                return workflow, ranges
            elif rule.value is not None:
                ranges[rule.compare] = get_overlap(
                    ranges[rule.compare], [1, rule.value] if rule.operator == ">" else [rule.value, 4000]
                )


def get_overlap(r1, r2):
    return r1 if len(r2) == 0 else [max(r1[0], r2[0]), min(r1[1], r2[1])]


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample.txt") as file:
        content = file.read()
        assert part1(content) == 19114
        assert part2(content) == 167409079868000
