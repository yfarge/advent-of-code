from typing import *

with open("./2022/day1/input.txt") as file:
    lines = file.readlines()


def mapElfToCalories(lines: List[str]):
    elfToCalories = {}
    cur_value = 0
    cur_elf = 1
    for line in lines:
        if (line != "\n"):
            cur_value += int(line)
            continue
        elfToCalories[cur_elf] = cur_value
        cur_value = 0
        cur_elf += 1

    return elfToCalories


def maxElf(elfToCalories: Dict):
    return max(elfToCalories.values())


elfToCalories = mapElfToCalories(lines)
# Part 1
print(
    f'Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?: {maxElf(elfToCalories)}')
