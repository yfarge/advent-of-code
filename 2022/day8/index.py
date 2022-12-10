from typing import *


with open("day8/input.txt") as file:
    tree_map = [[eval(c) for c in [*line.strip()]]
                for line in file.readlines()]

    ROWS, COLS = len(tree_map), len(tree_map[0])


def createDirectionalHeightMap():
    res = [[[0] * 4 for _ in range(COLS)] for _ in range(ROWS)]

    for j in range(1, ROWS - 1):
        for i in range(1, COLS - 1):
            res[j][i][0] = max(res[j - 1][i][0], tree_map[j - 1][i])
            res[j][i][1] = max(res[j][i - 1][1], tree_map[j][i - 1])
            y, x = ROWS - 1 - j, COLS - 1 - i
            res[y][x][2] = max(res[y + 1][x][2], tree_map[y + 1][x])
            res[y][x][3] = max(res[y][x + 1][3], tree_map[y][x + 1])

    return res


def partOne():
    directionalHeightMap = createDirectionalHeightMap()
    total = ROWS + ROWS + COLS + COLS - 4
    for j in range(1, ROWS - 1):
        for i in range(1, COLS - 1):
            if tree_map[j][i] > min(directionalHeightMap[j][i]):
                total += 1

    return total


print(
    f'Consider your map; how many trees are visible from outside the grid?: {partOne()}')
