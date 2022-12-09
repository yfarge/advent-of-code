from typing import *

with open("day6/input.txt") as file:
    datastream = file.read()
    PACKET_MARKER_LENGTH = 4
    MESSAGE_MARKER_LENGTH = 14
    assert(len(datastream) >= MESSAGE_MARKER_LENGTH)


def charactersProcessedBeforeFirstMarker(data: str, markerLength: int):
    for i in range(markerLength, len(data)):
        markerCandidate = set(data[i - markerLength: i])
        if (len(markerCandidate) == markerLength):
            return i


print(
    f'How many characters need to be processed before the first start-of-packet marker is detected?: {charactersProcessedBeforeFirstMarker(datastream, PACKET_MARKER_LENGTH)}')

print(
    f'How many characters need to be processed before the first start-of-message marker is detected?: {charactersProcessedBeforeFirstMarker(datastream, MESSAGE_MARKER_LENGTH)}')
