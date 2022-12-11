from typing import *

with open("day9/input.txt") as file:
    instructions = [line.strip().split(' ') for line in file.readlines()]


head_position = [0, 0]
tail_position = [0, 0]
visitedPositions = set()


def isTouching(a: List[int], b: List[int]):
    return pow(pow(a[0] - b[0], 2) + pow(a[1] - b[1], 2), 0.5) <= pow(2, 0.5)
