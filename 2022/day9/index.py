from typing import *

with open("day9/input.txt") as file:
    instructions = [(a, eval(b)) for a, b in [line.strip().split(' ')
                                              for line in file.readlines()]]

head_position = [0, 0]
tail_position = [0, 0]
visitedPositions = set()
visitedPositions.add((0, 0))

X_AXIS, Y_AXIS = 0, 1
direction2OffsetAxis = {
    "U": (-1, Y_AXIS),
    "L": (1, X_AXIS),
    "D": (1, Y_AXIS),
    "R": (-1, X_AXIS),

}


def calculateDistance(a: List[int], b: List[int]) -> complex:
    return pow(pow(a[0] - b[0], 2) + pow(a[1] - b[1], 2), 0.5)


def isTouching(a: List[int], b: List[int]) -> bool:
    return calculateDistance(a, b) <= pow(2, 0.5)


def calculateTailMove(eye: List[int], target: List[int]) -> Tuple[int, int]:
    return ((target[0] - eye[0]) // abs(target[0] - eye[0] or 1),
            (target[1] - eye[1]) // abs(target[1] - eye[1] or 1))


def interpretInstruction(instruction: Tuple[str, int]) -> Union[Tuple[int, int, int], ValueError]:
    direction, steps = instruction
    offset, axis = direction2OffsetAxis[direction]
    return (steps, offset, axis)


def executeInstructions() -> None:
    for instruction in instructions:
        steps, offset, axis = interpretInstruction(instruction)
        while steps > 0:
            head_position[axis] += offset
            if (not isTouching(tail_position, head_position)):
                dx, dy = calculateTailMove(tail_position, head_position)
                tail_position[0] += dx
                tail_position[1] += dy
                visitedPositions.add(tuple(tail_position))
            steps -= 1


def partOne() -> int:
    return len(visitedPositions)


executeInstructions()
print(partOne())
