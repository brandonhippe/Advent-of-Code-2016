import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2016 Day 3 Part 1

    >>> part1(['5 10 25'])
    0
    """

    return sum([1 if validTriangle(nums) else 0 for nums in [[int(x) for x in re.findall('\d+', line)] for line in data]])


def part2(data):
    """ 2016 Day 3 Part 2

    >>> part2(['101 301 501', '102 302 502', '103 303 503', '201 401 601', '202 402 602', '203 403 603'])
    6
    """

    nums = [[int(x) for x in re.findall('\d+', line)] for line in data]

    count = 0
    for j in range(3):
        arr = []
        for i in range(len(nums)):
            arr.append(nums[i][j])
            if len(arr) == 3:
                count += 1 if validTriangle(arr) else 0
                arr = []

    return count


def validTriangle(line):
    for i in range(len(line)):
        if sum(line[:i] + line[i + 1:]) <= line[i]:
            return False

    return True


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
        print(f"\nPart 1:\nPossible triangles: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPossible triangles: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)