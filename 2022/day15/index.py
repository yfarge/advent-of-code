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


def addPoints(point_one: Tuple[int, int], point_two: Tuple[int, int]):
    return tuple(map(lambda a, b: a + b, point_one, point_two))


def scalePoint(scalar: int, point: Tuple[int, int]):
    return tuple(map(lambda x: x * scalar, point))


def addCoverage(sensor: Tuple[int, int], beacon: Tuple[int, int], coverage_map: Dict[int, Set[Tuple[int, int]]]) -> List[Tuple[int, int]]:
    ray_length = manhattanDistance(sensor, beacon)
    direction_vectors = [(i, j) for i in range(-1, 2) for j in range(-1, 2)]
    grid = [["." for _ in range(25)] for _ in range(24)]
    grid[7][8] = "S"

    for scale in range(1, ray_length + 1):
        for direction in direction_vectors:
            point = addPoints(sensor, scalePoint(scale, direction))
            if manhattanDistance(sensor, point) > ray_length:
                continue
            grid[point[1]][point[0]] = "#"
            if point[1] not in coverage_map.keys():
                coverage_map[point[1]] = set()
            coverage_map[point[1]].add(point)


coverage_map = {}
addCoverage((8, 7), (2, 10), coverage_map)
for sensor, beacon in sensor_beacon:
    addCoverage(sensor, beacon, coverage_map)

print(len(coverage_map[10]))
