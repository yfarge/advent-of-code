from typing import *

with open("day9/input.txt") as file:
    instructions = [(a, eval(b)) for a, b in [line.strip().split(' ')
                                              for line in file.readlines()]]

X_AXIS, Y_AXIS = 0, 1
direction2OffsetAxis = {
    "U": (1, Y_AXIS),
    "L": (-1, X_AXIS),
    "D": (-1, Y_AXIS),
    "R": (1, X_AXIS),
}


def calculateDistance(a: Tuple[int], b: Tuple[int]) -> complex:
    return pow(pow(a[0] - b[0], 2) + pow(a[1] - b[1], 2), 0.5)


def isTouching(a: Tuple[int], b: Tuple[int]) -> bool:
    return calculateDistance(a, b) <= pow(2, 0.5)


def interpretInstruction(instruction: Tuple[str, int]) -> Union[Tuple[int, int, int], ValueError]:
    direction, steps = instruction
    offset, axis = direction2OffsetAxis[direction]
    return (steps, offset, axis)


def updatePositions(positions: List[Tuple[int, int]], offset: int, axis: int):
    new_head = list(positions[0])
    new_head[axis] += offset
    if (isTouching(positions[1], new_head)):
        positions[0] = tuple(new_head)
    else:
        positions.insert(0, tuple(new_head))
        positions.pop()


def partOne() -> int:
    positions = [(0, 0), (0, 0)]
    visited_positions = set([(0, 0)])
    for instruction in instructions:
        steps, offset, axis = interpretInstruction(instruction)
        for _ in range(steps):
            updatePositions(positions, offset, axis)
            visited_positions.add(tuple(positions[-1]))
    return len(visited_positions)


def partTwo() -> int:
    positions = [tuple([0, 0]) for _ in range(10)]
    visited_positions = set([(0, 0)])
    for instruction in instructions:
        steps, offset, axis = interpretInstruction(instruction)
        for _ in range(steps):
            updatePositions(positions, offset, axis)
            visited_positions.add(tuple(positions[-1]))
    return len(visited_positions)


print(
    f'How many positions does the tail of the rope visit at least once?: {partOne()}')

print(
    f'Simulate your complete series of motions on a larger rope with ten knots. How many positions does the tail of the rope visit at least once?: {partTwo()}')
