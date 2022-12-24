from typing import *
import re

with open("day15/input.txt") as file:
    lines = [line.strip() for line in file.readlines()]
    sensor_beacon = []
    for line in lines:
        positions = [int(v) for v in re.findall(r'\-?\d+', line)]
        sensor_beacon.append([tuple(positions[:2]), tuple(positions[2:])])
