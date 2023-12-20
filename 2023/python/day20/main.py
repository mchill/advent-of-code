import math
import os
import re
from dataclasses import dataclass

day = os.path.dirname(os.path.realpath(__file__))


@dataclass
class Module:
    name: str
    type: str
    state: str
    inputs: dict
    destinations: list


def part1(file: str):
    modules = get_modules(file)
    low_pulses = 0
    high_pulses = 0

    for _ in range(1000):
        pulses = [("button", "broadcaster", "low")]

        while pulses:
            (_, _, pulse) = pulses[0]

            if pulse == "low":
                low_pulses += 1
            elif pulse == "high":
                high_pulses += 1

            run_pulse(modules, pulses)

    return low_pulses * high_pulses


def part2(file: str):
    modules = get_modules(file)
    inputs = modules["tj"].inputs

    cycles = []

    for find_module in inputs.keys():
        modules = get_modules(file)
        cycle = 0

        while True:
            cycle += 1
            pulses = [("button", "broadcaster", "low")]

            while pulses:
                (from_module, module, pulse) = pulses[0]

                if module == "tj" and from_module == find_module and pulse == "high":
                    cycles.append(cycle)
                    break

                run_pulse(modules, pulses)
            else:
                continue
            break

    return math.lcm(*cycles)


def get_modules(file: str):
    modules = {}

    for line in file.splitlines():
        (type, name, destinations) = re.match(r"(%|&)?([a-z]+) -> (.+)", line).groups()
        modules[name] = Module(name, type, "off", {}, destinations.split(", "))

    for module in modules.values():
        for destination in module.destinations:
            if destination in modules:
                modules[destination].inputs[module.name] = "low"

    return modules


def run_pulse(modules, pulses):
    (from_module, module, pulse) = pulses.pop(0)

    if module not in modules:
        return
    module = modules[module]

    if module.type == None:
        pulses.extend([(module.name, destination, pulse) for destination in module.destinations])
    elif module.type == "%":
        if pulse == "high":
            return
        elif module.state == "off":
            module.state = "on"
            pulses.extend([(module.name, destination, "high") for destination in module.destinations])
        elif module.state == "on":
            module.state = "off"
            pulses.extend([(module.name, destination, "low") for destination in module.destinations])
    elif module.type == "&":
        module.inputs[from_module] = pulse
        if all(input == "high" for input in module.inputs.values()):
            pulses.extend([(module.name, destination, "low") for destination in module.destinations])
            return
        pulses.extend([(module.name, destination, "high") for destination in module.destinations])


if __name__ == "__main__":
    with open(day + "/input.txt") as file:
        content = file.read()
        print("Part 1: " + str(part1(content)))
        print("Part 2: " + str(part2(content)))


def test():
    with open(day + "/sample1.txt") as file:
        content = file.read()
        assert part1(content) == 32000000

    with open(day + "/sample2.txt") as file:
        content = file.read()
        assert part1(content) == 11687500
