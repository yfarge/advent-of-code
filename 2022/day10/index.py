from typing import *

with open("day10/input.txt") as file:
    instructions = [instruction.strip().split() for instruction in file.readlines()]


def populateRegisterValues():
    register_values = []
    x = 1
    for instruction in instructions:
        command = instruction[0]
        if (command == "noop"):
            register_values.append(x)
        else:
            register_values.extend([x, x])
            x += eval(instruction[1])
    register_values.append(x)
    return register_values

registerValues = populateRegisterValues()
def partOne():
    total = 0
    for i in range(19, len(registerValues), 40):
        total += (i + 1) * registerValues[i]
    return total

def partTwo():
    ROWS, COLS = 6, 40
    crt = [["." for _ in range(COLS)] for _ in range(ROWS)]

    for i, x in enumerate(registerValues):
        row_index = i // 40
        col_index = i % 40
        sprite_positions = [x - 1, x, x + 1]
        if (col_index in sprite_positions):
            crt[row_index][col_index] = "#"
    for row in crt:
        print(row)

print(f'Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is the sum of these six signal strengths?: {partOne()}')

print(f'Render the image given by your program. What eight capital letters appear on your CRT?:')
partTwo()