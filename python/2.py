import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2016 Day 2 Part 1

    >>> part1(['ULL', 'RRDDD', 'LURDL', 'UUUUD'])
    1985
    """

    keypad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

    moves = {'U': (0, -1), 'D': (0, 1), 'L': (-1, 0), 'R': (1, 0)}
    pos = (1, 1)
    code = 0
    for line in data:
        for c in line:
            newPos = tuple(p + o for p, o in zip(pos, moves[c]))
            if 0 <= min(newPos) and len(keypad) > max(newPos):
                pos = newPos

        code *= 10
        code += keypad[pos[1]][pos[0]]

    return code


def part2(data):
    """ 2016 Day 2 Part 2

    >>> part2(['ULL', 'RRDDD', 'LURDL', 'UUUUD'])
    '5DB3'
    """

    keypad = {(0, 0): '7', (1, 0): '8', (2, 0): '9', (-1, 0): '6', (-2, 0): '5', (0, 1): 'B', (0, 2): 'D', (0, -1): '3', (0, -2): '1', (-1, -1): '2', (1, -1): '4', (-1, 1): 'A', (1, 1): 'C'}

    moves = {'U': (0, -1), 'D': (0, 1), 'L': (-1, 0), 'R': (1, 0)}
    pos = (-2, 0)
    code = ''
    for line in data:
        for c in line:
            newPos = tuple(p + o for p, o in zip(pos, moves[c]))
            if newPos in keypad:
                pos = newPos

        code += keypad[pos]

    return code


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
        print(f"\nPart 1:\nCode: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCode: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)