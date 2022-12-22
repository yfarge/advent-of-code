from typing import *
import string
from heapq import heapify, heappush, heappop

with open("day12/input.txt") as file:
    elevation_map = [[*line.strip()] for line in file.readlines()]
    ROWS, COLS = len(elevation_map), len(elevation_map[0])
    decoded_elevation = {s: i for i, s in enumerate(string.ascii_lowercase)}
    decoded_elevation["S"] = 0
    decoded_elevation["E"] = 25


def createAdjacencyList():
    adjacency_list: Dict[Tuple[int, int], List[Tuple[int, int]]] = {}
    for row in range(ROWS):
        for col in range(COLS):
            node_decoded_elevation = decoded_elevation[elevation_map[row][col]]
            node_position = (row, col)
            adjacency_list[node_position] = []
            for direction in [(-1, 0), (0, -1), (1, 0), (0, 1)]:
                neighbor_row, neighbor_col = tuple(
                    map(lambda a, b: a + b, node_position, direction))
                if (neighbor_row >= 0 and neighbor_row < ROWS and
                        neighbor_col >= 0 and neighbor_col < COLS):
                    neighbor_elevation = elevation_map[neighbor_row][neighbor_col]
                    neighbor_decoded_elevation = decoded_elevation[neighbor_elevation]
                    if (node_decoded_elevation - neighbor_decoded_elevation) <= 1:
                        adjacency_list[node_position].append(
                            (neighbor_row, neighbor_col))
    return adjacency_list


adjacency_list = createAdjacencyList()
verticies = [(row, col) for row in range(ROWS) for col in range(COLS)]


def dijkstras(source: Tuple[int, int]):
    dist: dict[Tuple[int, int], int] = {}
    dist[source] = 0
    prev: dict[Tuple[int, int], Tuple[int, int]] = {}
    prev[source] = None

    queue = []
    heapify(queue)

    for v in verticies:
        if (v != source):
            dist[v] = float('inf')
            prev[v] = None
        heappush(queue, (dist[v], v))

    while len(queue) > 0:
        u = heappop(queue)[1]
        neighbors = adjacency_list[u]
        for v in neighbors:
            candidate_dist = dist[u] + 1
            if (candidate_dist < dist[v]):
                v_index = queue.index((dist[v], v))
                dist[v] = candidate_dist
                prev[v] = u
                queue[v_index] = (candidate_dist, v)
                heapify(queue)
    return dist, prev


def getStartAndEndPositions() -> Union[List[Tuple[int, int]], LookupError]:
    start, end = (-1, -1), (-1, -1)
    for row in range(ROWS):
        for col in range(COLS):
            if (elevation_map[row][col] == "S"):
                start = (row, col)
            if (elevation_map[row][col] == "E"):
                end = (row, col)
            if start != (-1, -1) and end != (-1, -1):
                return [start, end]
    raise LookupError("Could not find 'S' or 'E' in elevation map")


start, end = getStartAndEndPositions()
dist, _ = dijkstras(end)


def getDistToTargetFromSource(source: Tuple[int, int]) -> int:
    return dist[source]


def getElevationPositions(elevation: str):
    positions = []
    for row in range(ROWS):
        for col in range(COLS):
            if (elevation_map[row][col]) == elevation:
                positions.append((row, col))
    return positions


def getMinDistToTargetFromElevation(elevation: str) -> int:
    elevation_positions = getElevationPositions(elevation)
    min_dist = float('inf')
    for position in elevation_positions:
        if (dist[position] < min_dist):
            min_dist = dist[position]
    return min_dist

print(
    f'What is the fewest steps required to move from your current position to the location that should get the best signal?: {getDistToTargetFromSource(start)}')

print(
    f'What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?: {getMinDistToTargetFromElevation("a")}')
