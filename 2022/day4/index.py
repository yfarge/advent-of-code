from typing import *

with open("day4/input.txt") as file:
    pairs = [[[int(i) for i in interval.split("-")] for interval in pair.strip().split(',')]
             for pair in file.readlines()]


def isContained(a: List[int], b: List[int]):
    aStart, aEnd = a
    bStart, bEnd = b

    return bStart >= aStart and bEnd <= aEnd or \
        aStart >= bStart and aEnd <= bEnd


def isOverlap(a: List[int], b: List[int]):
    aStart, aEnd = a
    bStart, bEnd = b

    return aStart <= bEnd and bStart <= aEnd


def countPairs(comparator: Callable = lambda a, b: True):
    count = 0
    for pair in pairs:
        intervalOne, intervalTwo = pair
        if (comparator(intervalOne, intervalTwo)):
            count += 1
    return count


print(
    f'In how many assignment pairs does one range fully contain the other?: {countPairs(isContained)}')

print(
    f'In how many assignment pairs do the ranges overlap?: {countPairs(isOverlap)}')
