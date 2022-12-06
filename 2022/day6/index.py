from typing import *

with open("2022/day6/input.txt") as file:
    datastream = file.read()
    assert(len(datastream) >= 4)


def charactersProcessedBeforeFirstMarker(data: str):
    for i in range(4, len(data)):
        markerCandidate = set(data[i - 4: i])
        if (len(markerCandidate) == 4):
            return i


print(
    f'How many characters need to be processed before the first start-of-packet marker is detected?: {charactersProcessedBeforeFirstMarker(datastream)}')
