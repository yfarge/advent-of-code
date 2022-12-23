from typing import *

with open("day13/input.txt") as file:
    lines = [line.strip() for line in file.readlines()]


def parsePacketPairStream(packetPairStream: List[List[str]]):
    pairs = []
    for i in range(0, len(packetPairStream), 3):
        pairs.append([eval(packetPairStream[i]),
                      eval(packetPairStream[i + 1])])
    return pairs


def matchTypes(item_one: Union[int, List], item_two: Union[int, List]):
    type_one, type_two = type(item_one), type(item_two)
    if (type_one == type_two):
        return item_one, item_two, type_one

    item_one = [item_one] if type_one == int else item_one
    item_two = [item_two] if type_two == int else item_two

    return item_one, item_two, List


def isOrdered(packet_one, packet_two):
    UNDECIDED = -1

    def recur(l, r):
        i = 0
        for i in range(min(len(l), len(r))):
            l_top, r_top, lr_type = matchTypes(l[i], r[i])
            if (lr_type == int):
                if (l_top != r_top):
                    return l_top < r_top
            else:
                status = recur(l_top, r_top)
                if (status != UNDECIDED):
                    return status
        if (len(r) == len(l)):
            return UNDECIDED
        return len(r) > len(l)

    return recur(packet_one, packet_two)


def partOne(pairs: List):
    res = 0
    for i, pair in enumerate(pairs, start=1):
        l, r = pair
        if (isOrdered(l, r)):
            res += i
    return res


pairs = parsePacketPairStream(lines)

print(
    f'Determine which pairs of packets are already in the right order. What is the sum of the indices of those pairs?: {partOne(pairs)}')
