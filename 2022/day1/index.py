from typing import *

with open("./2022/day1/input.txt") as file:
    lines = file.readlines()


def createCalorieArray(lines: List[str]):
    calories = []
    cur_value = 0
    for line in lines:
        if (line != "\n"):
            cur_value += int(line)
            continue
        calories.append(cur_value)
        cur_value = 0

    return calories


def maxElfCalories(calories: List[int]):
    return max(calories)


def topThreeElfCalories(calories: List[int]):
    return sum(sorted(calories, reverse=True)[:3])


calories = createCalorieArray(lines)
# Part 1
print(
    f'Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?: {maxElfCalories(calories)}')
# Part 2
print(
    f'Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?: {topThreeElfCalories(calories)}')
