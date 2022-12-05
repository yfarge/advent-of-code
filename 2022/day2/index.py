from typing import *

with open("2022/day2/input.txt") as file:
    lines = file.readlines()


DRAW_VALUE = 3
WON_VALUE = 6

opponentToPlayer = {"A": "X", "B": "Y", "C": "Z"}
playerToOpponent = {v: k for k, v in opponentToPlayer.items()}
shapeValue = {"X": 1, "Y": 2, "Z": 3}
winningMove = {"A": "Y", "B": "Z", "C": "X"}
losingMove = {playerToOpponent[v]: opponentToPlayer[k]
              for k, v in winningMove.items()}


def isWinningMove(opponentMove, playerMove):
    return winningMove.get(opponentMove) == playerMove


def isDraw(opponentMove, playerMove):
    return opponentToPlayer.get(opponentMove) == playerMove


def calculateRoundScorePT1(opponentMove, playerMove):
    if (isDraw(opponentMove, playerMove)):
        return shapeValue.get(playerMove) + DRAW_VALUE
    if (isWinningMove(opponentMove, playerMove)):
        return shapeValue.get(playerMove) + WON_VALUE
    return shapeValue.get(playerMove)


def caluclateRoundScorePT2(opponentMove, outcome):
    # Lose
    if (outcome == "X"):
        return shapeValue.get(losingMove.get(opponentMove))
    # Draw
    if (outcome == "Y"):
        return shapeValue.get(opponentToPlayer.get(opponentMove)) + DRAW_VALUE
    return shapeValue.get(winningMove.get(opponentMove)) + WON_VALUE


def totalStrategyScore(scoringFunction: Callable):
    total = 0
    for line in lines:
        opponentMove, secondColumn = line.strip().split(" ")
        total += scoringFunction(opponentMove, secondColumn)
    return total


print(
    f'What would your total score be if everything goes exactly according to your strategy guide?: {totalStrategyScore(calculateRoundScorePT1)}')
print(
    f'Following the Elf\'s instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?: {totalStrategyScore(caluclateRoundScorePT2)}')
