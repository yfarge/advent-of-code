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


def executeInstructions(stacks: List[List[str]], instructions: List[List[int]], singleMove: bool = False):
    initialStacks = [s.copy() for s in stacks]
    for instruction in instructions:
        cratesToMove, currentStack, targetStack = instruction
        crates = initialStacks[currentStack - 1][-cratesToMove:]
        if singleMove:
            crates = crates[::-1]
        del initialStacks[currentStack - 1][-cratesToMove:]
        initialStacks[targetStack - 1].extend(crates)
    return initialStacks


def getTopOfEachStack(stacks: List[List[str]]):
    return "".join([s[-1] for s in stacks])


print(
    f'After the rearrangement procedure completes, what crate ends up on top of each stack?: {getTopOfEachStack(executeInstructions(stacks, instructions, True))}')

print(
    f'After the rearrangement procedure completes, what crate ends up on top of each stack?: {getTopOfEachStack(executeInstructions(stacks, instructions))}')
