import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import hashlib


def part1(data):
    """ 2016 Day 17 Part 1

    >>> part1(['ihgpwlah'])
    'DDRRRD'
    >>> part1(['kglvqrro'])
    'DDUDRLRRUDRD'
    >>> part1(['ulqzkmiv'])
    'DRURDRUDDLLDLUURRDULRLDUUDDDRR'
    """

    word = data[0]
    openList = [['', (0, 0)]]

    while len(openList) != 0:
        currPath, currPos = openList.pop(0)

        if currPos == (3, 3):
            return currPath

        for d, n in zip('UDLR', hashlib.md5(f'{word}{currPath}'.encode()).hexdigest()[:4]):
            if n not in 'bcdef':
                continue

            pos = tuple(p + o for p, o in zip(currPos, OFFSETS[d]))
            if min(pos) < 0 or max(pos) > 3:
                continue

            openList.append([currPath + d, pos])

    return -1


def part2(data):
    """ 2016 Day 17 Part 2

    >>> part2(['ihgpwlah'])
    370
    >>> part2(['kglvqrro'])
    492
    >>> part2(['ulqzkmiv'])
    830
    """

    word = data[0]
    openList = [['', (0, 0)]]
    maxPath = 0

    while len(openList) != 0:
        currPath, currPos = openList.pop(0)

        if currPos == (3, 3):
            if len(currPath) > maxPath:
                maxPath = len(currPath)

            continue

        for d, n in zip('UDLR', hashlib.md5(f'{word}{currPath}'.encode()).hexdigest()[:4]):
            if n not in 'bcdef':
                continue

            pos = tuple(p + o for p, o in zip(currPos, OFFSETS[d]))
            if min(pos) < 0 or max(pos) > 3:
                continue

            openList.append([currPath + d, pos])

    return maxPath


OFFSETS = {'U': (0, -1), 'D': (0, 1), 'L': (-1, 0), 'R': (1, 0)}


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
        print(f"\nPart 1:\nShortest path to vault: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLongest path to vault: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)