from typing import *

with open("2022/day5/input.txt") as file:
    lines = file.readlines()
    stacks = [[] for _ in range(9)]
    instructions = []

    for line in lines[:8]:
        stackSlice = [*line[1::4]]
        for i, crate in enumerate(stackSlice):
            if (crate != ' '):
                stacks[i].append(crate)
    for line in lines[10:]:
        instructions.append([int(s) for s in line.split() if s.isdigit()])

    stacks = [s[::-1] for s in stacks]


def executeInstructions(initialStacks: List[List[str]], instructions: List[List[int]]):
    for instruction in instructions:
        cratesToMove, currentStack, targetStack = instruction

        while cratesToMove > 0:
            crate = initialStacks[currentStack - 1].pop()
            initialStacks[targetStack - 1].append(crate)
            cratesToMove -= 1
    return initialStacks


def getTopOfEachStack(stacks: List[List[str]]):
    return "".join([s[-1] for s in stacks])


print(
    f'After the rearrangement procedure completes, what crate ends up on top of each stack?: {getTopOfEachStack(executeInstructions(stacks, instructions))}')
