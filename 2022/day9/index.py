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


def clamp(v: int, min_value: int, max_value: int):
    return max(min(v, max_value), min_value)


def calculateDistance(a: Tuple[int], b: Tuple[int]) -> complex:
    return pow(pow(a[0] - b[0], 2) + pow(a[1] - b[1], 2), 0.5)


def isTouching(a: Tuple[int], b: Tuple[int]) -> bool:
    return calculateDistance(a, b) <= pow(2, 0.5)


def lookUpDirectionOffset(direction: str) -> Union[Tuple[int, int], ValueError]:
    match direction:
        case "U":
            return (0, 1)
        case "L":
            return (-1, 0)
        case "D":
            return (0, -1)
        case "R":
            return (1, 0)
        case _:
            raise ValueError(
                f'Direction not recognized, expected U | L | D | R, got {direction}')


def interpretInstruction(instruction: Tuple[str, int]) -> Tuple[int, int, int]:
    direction, steps = instruction
    offset = lookUpDirectionOffset(direction)
    return (steps, offset)


def simulateHeadTailPair(head: List[int], tail: List[int]) -> None:
    if (isTouching(head, tail)):
        return
    toward = [clamp(h - t, -1, 1) for h, t in zip(head, tail)]
    tail[:] = [a + b for a, b in zip(tail, toward)]


def simulateRope(positions: List[List[int]]) -> None:
    for i in range(len(positions) - 1):
        simulateHeadTailPair(positions[i], positions[i + 1])


def calculateTailPositions(numKnots: int):
    assert numKnots >= 2, f'Expected numKnots >= 2, received {numKnots}'
    knots = [[0, 0] for _ in range(numKnots)]
    visited_positions = set([(0, 0)])
    for instruction in instructions:
        steps, offset = interpretInstruction(instruction)
        for _ in range(steps):
            knots[0][:] = [a + b for a, b in zip(knots[0], offset)]
            simulateRope(knots)
            visited_positions.add(tuple(knots[-1]))
    return len(visited_positions)


print(
    f'How many positions does the tail of the rope visit at least once?: {calculateTailPositions(2)}')

print(
    f'Simulate your complete series of motions on a larger rope with ten knots. How many positions does the tail of the rope visit at least once?: {calculateTailPositions(10)}')
