from typing import *

with open("2022/day2/input.txt") as file:
    lines = file.readlines()


DRAW_VALUE = 3
WON_VALUE = 6

shapeValue = {"X": 1, "Y": 2, "Z": 3}
winningMove = {"A": "Y", "B": "Z", "C": "X"}
opponentToPlayer = {"A": "X", "B": "Y", "C": "Z"}


def isWinningMove(opponentMove, playerMove):
    return winningMove.get(opponentMove) == playerMove


def isDraw(opponentMove, playerMove):
    return opponentToPlayer.get(opponentMove) == playerMove


def calculateRoundScore(opponentMove, playerMove):
    if (isDraw(opponentMove, playerMove)):
        return shapeValue.get(playerMove) + DRAW_VALUE
    if (isWinningMove(opponentMove, playerMove)):
        return shapeValue.get(playerMove) + WON_VALUE
    return shapeValue.get(playerMove)


def totalStrategyScore():
    total = 0
    for line in lines:
        opponentMove, playerMove = line.strip().split(" ")
        total += calculateRoundScore(opponentMove, playerMove)
    return total


print(
    f'What would your total score be if everything goes exactly according to your strategy guide?: {totalStrategyScore()}')
