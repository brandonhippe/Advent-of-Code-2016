import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data, rows = 40):
    """ 2016 Day 18 Part 1

    >>> part1(['.^^.^.^^^^'], 10)
    38
    """

    first = '.' + data[0] + '.'

    safe = len([t for t in first if t == '.']) - 2
    pRow = first

    for _ in range(1, rows):
        rowText = ''
        for i in range(1, len(first) - 1):
            if pRow[i-1:i+2] in TILE_RULES:
                rowText += '^'
            else:
                rowText += '.'
                safe += 1

        pRow = '.' + rowText + '.'

    return safe


def part2(data):
    """ 2016 Day 18 Part 2
    """

    first = '.' + data[0] + '.'

    safe = len([t for t in first if t == '.']) - 2
    pRow = first

    for _ in range(1, 400000):
        rowText = ''
        for i in range(1, len(first) - 1):
            if pRow[i-1:i+2] in TILE_RULES:
                rowText += '^'
            else:
                rowText += '.'
                safe += 1

        pRow = '.' + rowText + '.'

    return safe


TILE_RULES = {'^^.', '.^^', '^..', '..^'}


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
        print(f"\nPart 1:\n{p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n{p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)