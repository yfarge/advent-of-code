from typing import *

with open("day13/input.txt") as file:
    lines = [[*line.strip()] for line in file.readlines()]


def parsePacket(packetInput: List[str]):
    packet = temp = []
    stack = []
    for char in packetInput:
        if char.isnumeric():
            temp.append(int(char))
        match char:
            case "[":
                stack.append(temp)
                temp.append([])
                temp = temp[-1]
            case "]":
                temp = stack.pop()
    return packet[0]


def parsePacketPairStream(packetPairStream: List[List[str]]):
    pairs = []
    for i in range(0, len(lines), 3):
        pairs.append([parsePacket(lines[i]),
                      parsePacket(lines[i + 1])])
    return pairs


def matchTypes(item_one: Union[int, List], item_two: Union[int, List]):
    type_one, type_two = type(item_one), type(item_two)
    if (type_one == int and type_two == list):
        item_one = [item_one]
    elif (type_two == int and type_one == list):
        item_two = [item_two]
    return item_one, item_two


def isInOrder(packet_one, packet_two):
    i = 0
    while i < len(packet_one) and i < len(packet_two):
        packet_one[i], packet_two[i] = matchTypes(packet_one[i], packet_two[i])
        type_one, type_two = type(packet_one[i]), type(packet_two[i])

        # Same type check
        if (type_one == int and type_two == int):
            if packet_one[i] > packet_two[i]:
                return False
        elif (type_one == list and type_two == list):
            if (not isInOrder(packet_one[i], packet_two[i])):
                return False
        i += 1
    return True


item_one = [9]
item_two = [[8, 7, 6]]

print(isInOrder(item_one, item_two))
pairs = parsePacketPairStream(lines)
