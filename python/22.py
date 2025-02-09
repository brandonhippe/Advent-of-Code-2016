import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
import heapq


def part1(data):
    """ 2016 Day 22 Part 1
    """

    goalNode = (-1, 0)
    nodes = {}
    for line in data[2:]:
        x, y, *otherNums = [int(n) for n in re.findall('\d+', line)]
        nodes[(x, y)] = Node(x, y, otherNums)
        if y == 0 and x > goalNode[0]:
            goalNode = (x, y)

    pairs = 0
    for n1 in nodes.values():
        for n2 in nodes.values():
            if n1 == n2:
                continue

            pairs += 1 if n1.used[0] > 0 and n2.avail >= n1.used[0] else 0

    return pairs


def part2(data):
    """ 2016 Day 22 Part 2

    >>> part2(['root@ebhq-gridcenter# df -h', 'Filesystem              Size  Used  Avail  Use%', '/dev/grid/node-x0-y0   10T    8T     2T   80%', '/dev/grid/node-x0-y1   11T    6T     5T   54%', '/dev/grid/node-x0-y2   32T   28T     4T   87%', '/dev/grid/node-x1-y0    9T    7T     2T   77%', '/dev/grid/node-x1-y1    8T    0T     8T    0%', '/dev/grid/node-x1-y2   11T    7T     4T   63%', '/dev/grid/node-x2-y0   10T    6T     4T   60%', '/dev/grid/node-x2-y1    9T    8T     1T   88%', '/dev/grid/node-x2-y2    9T    6T     3T   66%'])
    7
    """

    goalNode = (-1, 0)
    nodes = {}
    for line in data[2:]:
        x, y, *otherNums = [int(n) for n in re.findall('\d+', line)]
        nodes[(x, y)] = Node(x, y, otherNums)
        if y == 0 and x > goalNode[0]:
            goalNode = (x, y)

    return dataPath(nodes, goalNode)


class Node:
    def __init__(self, x, y, inpNums):
        self.startPos = (x, y)
        self.size, self.used, self.avail, self.percent = inpNums
        self.used = (self.used, self.startPos)

    def __lt__(self, other):
        return self.percent < other.percent
    

def manhatDist(p1, p2):
    return sum(abs(c1 - c2) for c1, c2 in zip(p1, p2))


def aStar(nodes, start, end, avgLen):
    openList = [[manhatDist(start, end), 0, start]]
    visited = {}

    while len(openList) != 0:
        currF, currG, currPos = heapq.heappop(openList)

        if currPos == end:
            return currG

        for n in [tuple(p + o for p, o in zip(currPos, offset)) for offset in [[0, 1], [0, -1], [1, 0], [-1, 0]]]:
            if n not in nodes or len(str(nodes[n].used[0])) > avgLen:
                continue

            nH, nG = manhatDist(n, end), currG + 1
            nF = nH + nG

            if n in visited and visited[n] <= nF:
                continue

            continuing = False
            for o in openList:
                if n == o[2] and nF >= o[0]:
                    continuing = True
                    break

            if continuing:
                continue

            heapq.heappush(openList, [nF, nG, n])

        visited[currPos] = currF

    return -1


def dataPath(nodes, goalNode):
    avgUsedLen = len(str(sum(n.used[0] for n in nodes.values()) // len(nodes)))
    minNode = list(nodes.keys())[[n.used[0] for n in nodes.values()].index(min(n.used[0] for n in nodes.values()))]

    steps = aStar(nodes, minNode, goalNode, avgUsedLen)
    while goalNode != (1, 0):
        steps += 5
        goalNode = (goalNode[0] - 1, 0)

    return steps


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
        print(f"\nPart 1:\nNumber of viable pairs of nodes: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFewest steps to access data: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)