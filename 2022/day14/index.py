from typing import *

with open("day14/input.txt") as file:
    rock_lines = [list(map(lambda x: tuple(map(lambda v: int(v), x.strip().split(","))), split_line)) for split_lines in [[line.strip().split("->")
                                                                                                                           for line in file.readlines()]] for split_line in split_lines]


def clamp(v: int, min_value: int, max_value: int) -> int:
    return max(min(v, max_value), min_value)


def addPoints(a: Tuple[int, int], b: Tuple[int, int]) -> Tuple[int, int]:
    return tuple(map(lambda x, y: x + y, a, b))


def interpolatePoints(a: Tuple[int, int], b: Tuple[int, int]) -> List[Tuple[int, int]]:
    points = [a]
    direction_vector = [clamp(a - b, -1, 1) for a, b in zip(b, a)]
    while a != b:
        a = tuple(map(lambda a, b: a + b, a, direction_vector))
        points.append(a)
    return points


def createInitialFilledTiles() -> Set[Tuple[int, int]]:
    filled_tiles = set()
    for line in rock_lines:
        for i in range(len(line) - 1):
            interpolated_points = interpolatePoints(
                line[i], line[i + 1])
            for point in interpolated_points:
                filled_tiles.add(point)
    return filled_tiles


def simulateSand() -> int:
    count = 0
    filled_points = createInitialFilledTiles()
    max_height = max(list(filled_points), key=lambda x: x[1])[1]
    position = (500, 0)
    while True:
        if (position[1] > max_height):
            break
        down = addPoints(position, (0, 1))
        left = addPoints(position, (-1, 1))
        right = addPoints(position, (1, 1))
        if (down not in filled_points):
            position = down
        elif (left not in filled_points):
            position = left
        elif (right not in filled_points):
            position = right
        else:
            filled_points.add(position)
            position = (500, 0)
            count += 1
    return count


print(
    f'Using your scan, simulate the falling sand. How many units of sand come to rest before sand starts flowing into the abyss below?: {simulateSand()}')
