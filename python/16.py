import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data, length = 272):
    """ 2016 Day 16 Part 1

    >>> part1(['10000'], 20)
    '01100'
    """

    return checkSum(data[0], length)


def part2(data):
    """ 2016 Day 16 Part 2
    """

    return checkSum(data[0], 35651584)


def dragonCurve(data, disk):
    while len(data) < disk:
        a = data
        b = ''.join('1' if c == '0' else '0' for c in a)
        data = a + '0' + b[::-1]

    return data


def checkSum(data, disk):
    data = dragonCurve(data, disk)
    while True:
        cs = ''
        for i in range(0, min(disk, len(data)), 2):
            cs += '1' if len(set(data[i:i+2])) == 1 else '0'

        data = cs
        if len(data) % 2 == 1:
            break

    return data


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
        print(f"\nPart 1:\nChecksum: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nChecksum: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)