from typing import *
import string

with open("2022/day3/input.txt") as file:
    rucksacks = file.readlines()

itemPriorities = {v: i for i, v in enumerate(
    list(string.ascii_letters), start=1)}


def findOverlap(compartmentOne, compartmentTwo) -> str:
    sortedCompartmentOne = "".join(
        sorted([*compartmentOne], key=lambda x: itemPriorities[x]))
    sortedCompartmentTwo = "".join(
        sorted([*compartmentTwo], key=lambda x: itemPriorities[x]))
    n = len(sortedCompartmentOne)
    i = j = 0

    while (i + j < n + n - 1):
        charOne = sortedCompartmentOne[i]
        charTwo = sortedCompartmentTwo[j]
        if (charOne == charTwo):
            return charOne
        if (i != n - 1 and itemPriorities[charOne] < itemPriorities[charTwo]):
            i = i + 1 if i + 1 < n else i
        else:
            j = j + 1 if j + 1 < n else j
    return ''


def sumOverlap():
    total = 0
    for rucksack in rucksacks:
        formattedRucksack = rucksack.strip()
        n = len(formattedRucksack)
        compartmentOne = formattedRucksack[: n // 2]
        compartmentTwo = formattedRucksack[n // 2:]
        total += itemPriorities[findOverlap(compartmentOne, compartmentTwo)]
    return total


print(
    f'Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?: {sumOverlap()}')
