from typing import *
import re

with open("day11/input.txt") as file:
    lines = [line.strip() for line in file.readlines() if line != "\n"]

# Monkey Class
    # List of starting items
    # Operation function that
    # takes in a worry level
    # applies an operation and divides the result by three
    # Test function that
    # takes in a worry level
    # checks if it passes the test
    # True: return index of monkey
    # False: return index of other monkey

# parent(value):
    # child(worry):
    # worry + value


class Monkey:
    def __init__(self, _items: List[int], _operation: Callable[[int], int], _test: Callable[[int], int], _divisor: int):
        self.items = _items
        self.operation = _operation
        self.test = _test
        self.inspected = 0
        self.divisor = _divisor

    def inspectItem(self, item: int):
        return self.operation(item) // 3

    def testItem(self, item: int):
        return self.test(item)


def createMonkeyOperation(operation: str, value: Union[str, int]) -> Callable[[int], int]:
    def monkeyOperation(worry: int) -> int:
        nonlocal value
        modifier = worry
        if value.isnumeric():
            modifier = int(value)

        match operation:
            case "-":
                return (worry - modifier)
            case "+":
                return (worry + modifier)
            case "*":
                return (worry * modifier)
            case "/":
                return (worry // modifier)
            case _:
                return worry
    return monkeyOperation


def createTest(divisor: int, true_index: int, false_index: int) -> Callable[[int], int]:
    def test(worry: int) -> int:
        if ((worry % divisor) == 0):
            return true_index
        return false_index
    return test


def createTroop():
    troop: List[Monkey] = []
    operation_regex = re.compile(
        r'^Operation: new = old ([\+\-\*\/] (?:\d+|old))$')
    integer_regex = re.compile(r'\d+')
    for i in range(0, len(lines), 6):
        items = [int(n) for n in integer_regex.findall(lines[i + 1])]
        op, value = operation_regex.findall(lines[i + 2])[0].split()
        divisor = int(integer_regex.findall(lines[i + 3])[0])
        true_monkey = int(integer_regex.findall(lines[i + 4])[0])
        false_monkey = int(integer_regex.findall(lines[i + 5])[0])

        operation = createMonkeyOperation(op, value)
        test = createTest(divisor, true_monkey, false_monkey)
        troop.append(Monkey(items, operation, test, divisor))
    return troop


def simulateRounds(troop: List[Monkey], rounds: int):
    def simulateRound(troop: List[Monkey]):
        for monkey in troop:
            for item in monkey.items:
                item = monkey.inspectItem(item)
                troop[monkey.testItem(item)].items.append(item)
            monkey.inspected += len(monkey.items)
            monkey.items.clear()

    while rounds > 0:
        simulateRound(troop)
        rounds -= 1


def calculateMonkeyBusiness(rounds: int):
    troop = createTroop()
    simulateRounds(troop, rounds)
    inspected = sorted([monkey.inspected for monkey in troop], reverse=True)

    return inspected[0] * inspected[1]


print(
    f'What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?: {calculateMonkeyBusiness(20)}')
