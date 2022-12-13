from typing import *

with open("day10/input.txt") as file:
        instructions = [instruction.strip().split() for instruction in file.readlines()]


def populateRegisterValues():
    register_values = []
    x = 1
    for instruction in instructions:
        command = instruction[0]
        if (command == "noop"):
            register_values.append(x)
        else:
            register_values.extend([x, x])
            x += eval(instruction[1])
    register_values.append(x)
    return register_values

def partOne():
    registerValues = populateRegisterValues()
    total = 0
    for i in range(19, len(registerValues), 40):
        total += (i + 1) * registerValues[i]
    return total

print(partOne())