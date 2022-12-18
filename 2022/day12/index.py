from typing import *
from sys import maxsize, setrecursionlimit
import string

setrecursionlimit(2000)
with open("day12/input.txt") as file:
    grid = [row.strip() for row in file.readlines()]
    ROWS, COLS = len(grid), len(grid[0])


def findStartAndEndCoordinates(grid: List[List[str]]) -> List[Tuple[int, int]]:
    start, end = (-1, -1), (-1, -1)
    for j in range(ROWS):
        for i in range(COLS):
            if grid[j][i] == "S":
                start = (j, i)
            elif grid[j][i] == "E":
                end = (j, i)
        if start != (-1, -1) and end != (-1, -1):
            return [start, end]
    return [start, end]


elevationToInt = {s: i for i, s in enumerate(list(string.ascii_lowercase))}
elevationToInt["S"] = elevationToInt['a']
elevationToInt["E"] = elevationToInt['z']


def findMinimumPath(grid: List[List[str]]):
    start, end = findStartAndEndCoordinates(grid)
    memo = {}
    path = set()

    def dfs(coord: Tuple[int, int], previous_elevation: int, steps: int):
        if (coord, previous_elevation, steps) in memo:
            return memo[(coord, previous_elevation, steps)]
        row, col = coord
        if (row >= ROWS or row < 0 or
            col >= COLS or col < 0 or
                elevationToInt[grid[row][col]] - previous_elevation > 1 or coord in path):
            return maxsize
        if (coord == end):
            return steps

        current_elevation = elevationToInt[grid[row][col]]

        path.add(coord)
        res = min(dfs((row - 1, col), current_elevation, steps + 1), dfs((row + 1, col), current_elevation, steps + 1),
                  dfs((row, col - 1), current_elevation, steps + 1), dfs((row, col + 1), current_elevation, steps + 1))
        path.remove(coord)
        memo[(coord, previous_elevation, steps)] = res
        return res

    return dfs(start, 0, 0)


print(
    f'What is the fewest steps required to move from your current position to the location that should get the best signal?: {findMinimumPath(grid)}')
