from typing import List, Tuple

with open("./input.txt") as file:
    machines = []
    for section in file.read().strip().split("\n\n"):
        machine = []
        for line in section.split("\n"):
            _, offsets = line.split(":")
            x, y = offsets.split(",")
            machine.append((int(x.strip()[2:]), int(y.strip()[2:])))
        machines.append(machine)


def backtrack(machine: List[Tuple[int, int]]):
    def dfs(node: Tuple[int, int], total: int):
        if node == target:
            return total

        if node in memo:
            return memo[node]

        if node[0] > target[0] or node[1] > target[1]:
            return float('inf')

        left = dfs((node[0] + a[0], node[1] + a[1]), total + 3)
        right = dfs((node[0] + b[0], node[1] + b[1]), total + 1)

        memo[node] = min(left, right)

        return memo[node]

    memo = {}
    a, b, target = machine

    return dfs((0, 0), 0)


def part1():
    total = 0
    for machine in machines:
        tokens = backtrack(machine)
        total += tokens if tokens != float('inf') else 0
    return total


print(part1())
