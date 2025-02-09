import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2016 Day 15 Part 1

    >>> part1(['Disc #1 has 5 positions; at time=0, it is at position 4.', 'Disc #2 has 2 positions; at time=0, it is at position 1.'])
    5
    """

    discs = []
    for line in data:
        discNum, positions, _, start = [int(x) for x in re.findall('-?\d+', line)]
        discs.append(Disc(discNum, positions, start))

    return fallTime(discs)


def part2(data):
    """ 2016 Day 15 Part 2
    """

    discs = []
    for line in data:
        discNum, positions, _, start = [int(x) for x in re.findall('-?\d+', line)]
        discs.append(Disc(discNum, positions, start))

    return fallTime(discs + [Disc(len(discs) + 1, 11, 0)])


class Disc:
    def __init__(self, num, positions, start):
        self.num = num
        self.numPos = positions
        self.pos = start


def fallTime(discs):
    i = 0
    while True:
        valid = True
        for disc in discs:
            if (i + disc.num + disc.pos) % disc.numPos != 0:
                valid = False
                break

        if valid:
            break

        i += 1

    return i


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
        print(f"\nPart 1:\nFirst time when capsule falls through: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst time when capsule falls through: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)