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


def partOne(pairs: List) -> int:
    res = 0
    for i, pair in enumerate(pairs, start=1):
        l, r = pair
        if (isOrdered(l, r)):
            res += i
    return res


def flattenPairs(pairs: List) -> List:
    return [packet for pair in pairs for packet in pair]


def sortPackets(packets: List) -> None:
    n = len(packets)
    for i in range(n - 1):
        for j in range(0, n - i - 1):
            if (not isOrdered(packets[j], packets[j+1])):
                packets[j], packets[j+1] = packets[j+1], packets[j]


def calculateDecoderKey(sorted_packets: List) -> int:
    res = 1
    for i, packet in enumerate(sorted_packets, start=1):
        if packet == [[2]]:
            res *= i
        elif packet == [[6]]:
            return res * i
    return res


def partTwo(pairs: List):
    packets = flattenPairs(pairs)
    packets.extend([[[2]], [[6]]])
    sortPackets(packets)
    return calculateDecoderKey(packets)


pairs = parsePacketPairStream(lines)

print(
    f'Determine which pairs of packets are already in the right order. What is the sum of the indices of those pairs?: {partOne(pairs)}')

print(
    f'Organize all of the packets into the correct order. What is the decoder key for the distress signal?: {partTwo(pairs)}'
)
