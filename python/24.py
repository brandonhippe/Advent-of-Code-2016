import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import heapq


def part1(data):
    """ 2016 Day 24 Part 1

    >>> part1(['###########', '#0.1.....2#', '#.#######.#', '#4.......3#', '###########'])
    14
    """

    numPos = {}
    positions = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l != '#':
                positions.add((x, y))
            
            if l in '0123456789':
                numPos[(x, y)] = int(l)

    adjList = [[None] * len(numPos) for _ in range(len(numPos))]
    for k1, v1 in zip(numPos.keys(), numPos.values()):
        for k2, v2 in zip(numPos.keys(), numPos.values()):
            if adjList[v1][v2] is None:
                d = aStar(k1, k2, positions, numPos)
                adjList[v1][v2] = d
                adjList[v2][v1] = d

    for k in range(len(adjList)):
        for i in range(len(adjList)):
            for j in range(len(adjList)):
                adjList[i][j] = min(adjList[i][j], adjList[i][k] + adjList[k][j])

    return shortestPathP1(adjList, 0, [False] * len(adjList))


def part2(data):
    """ 2016 Day 24 Part 2
    """

    numPos = {}
    positions = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l != '#':
                positions.add((x, y))
            
            if l in '0123456789':
                numPos[(x, y)] = int(l)

    adjList = [[None] * len(numPos) for _ in range(len(numPos))]
    for k1, v1 in zip(numPos.keys(), numPos.values()):
        for k2, v2 in zip(numPos.keys(), numPos.values()):
            if adjList[v1][v2] is None:
                d = aStar(k1, k2, positions, numPos)
                adjList[v1][v2] = d
                adjList[v2][v1] = d

    for k in range(len(adjList)):
        for i in range(len(adjList)):
            for j in range(len(adjList)):
                adjList[i][j] = min(adjList[i][j], adjList[i][k] + adjList[k][j])

    return shortestPathP2(adjList, 0, [False] * len(adjList))


def manhatDist(p1, p2):
    return sum(abs(c1 - c2) for c1, c2 in zip(p1, p2))


def aStar(start, end, positions, numPos):
    openList = [[manhatDist(start, end), 0, start]]
    visited = {}

    while len(openList) != 0:
        currF, currG, pos = heapq.heappop(openList)
        
        if pos == end:
            return currG

        for n in [tuple(p + o for p, o in zip(pos, offset)) for offset in [[1, 0], [-1, 0], [0, 1], [0, -1]]]:
            if n not in positions or (n != end and n in numPos):
                continue

            nH, nG = manhatDist(n, end), currG + 1
            nF = nH + nG

            if n in visited and visited[n] <= nF:
                continue

            continuing = False
            for o in openList:
                if o[-1] == n and o[0] <= nF:
                    continuing = True
                    break

            if continuing:
                continue

            heapq.heappush(openList, [nF, nG, n])

        visited[pos] = currF

    return float('inf')


def shortestPathP1(adjList, start, visited):
    visited[start] = True
    if False not in visited:
        visited[start] = False
        return 0

    shortest = float('inf')
    for n, l in enumerate(adjList[start]):
        if n == start or visited[n] or l < 0:
            continue

        pathLen = shortestPathP1(adjList, n, visited) + l
        if pathLen < shortest:
            shortest = pathLen

    visited[start] = False

    return shortest


def shortestPathP2(adjList, start, visited):
    visited[start] = True
    if False not in visited:
        visited[start] = False
        return adjList[start][0]

    shortest = float('inf')
    for n, l in enumerate(adjList[start]):
        if n == start or visited[n]:
            continue

        pathLen = shortestPathP2(adjList, n, visited) + l
        if pathLen < shortest:
            shortest = pathLen

    visited[start] = False

    return shortest


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nFewest steps to reach all numbers: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPossilbe fewest steps to reach all numbers and return to 0: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)