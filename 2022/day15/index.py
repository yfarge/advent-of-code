from typing import *
import re

with open("day15/input.txt") as file:
    lines = [line.strip() for line in file.readlines()]
    sensor_beacon = []
    for line in lines:
        positions = [int(v) for v in re.findall(r'\-?\d+', line)]
        sensor_beacon.append([tuple(positions[:2]), tuple(positions[2:])])


def manhattanDistance(point_one: Tuple[int, int], point_two: Tuple[int, int]) -> int:
    return sum([abs(a - b) for a, b in zip(point_one, point_two)])
