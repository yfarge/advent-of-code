from typing import *
import string

with open("2022/day3/input.txt") as file:
    rucksacks = file.readlines()

itemPriorities = {v: i for i, v in enumerate(
    list(string.ascii_letters), start=1)}


def findOverlap(*args):
    sortedStrings = [sorted([*arg], key=lambda x: itemPriorities[x])
                     for arg in args]

    lowestPrio = float('inf')
    pendingRemoval = 0
    while all([len(s) > 0 for s in sortedStrings]):
        if (all([s[0] == sortedStrings[0][0] for s in sortedStrings])):
            return sortedStrings[0][0]
        for i in range(len(sortedStrings)):
            prio = itemPriorities[sortedStrings[i][0]]
            if (prio < lowestPrio):
                lowestPrio = prio
                pendingRemoval = i
        sortedStrings[pendingRemoval].pop(0)
        pendingRemoval = 0
        lowestPrio = float('inf')
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


def sumBadgePriorites():
    total = 0
    for i in range(0, len(rucksacks), 3):
        r1 = rucksacks[i].strip()
        r2 = rucksacks[i + 1].strip()
        r3 = rucksacks[i + 2].strip()
        total += itemPriorities[findOverlap(r1, r2, r3)]
    return total


print(
    f'Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?: {sumOverlap()}')


print(
    f'Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?: {sumBadgePriorites()}')
