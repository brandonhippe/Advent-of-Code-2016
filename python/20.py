import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data, allowed = {(0, 4294967295)}):
    """ 2016 Day 20 Part 1

    >>> part1(['5-8', '0-2', '4-7'], {(0, 9)})
    3
    """

    lines = set(tuple(int(x) for x in re.findall('\d+', line)) for line in data)

    while len(lines) > 0:
        low, high = list(lines)[0]
        lines.remove((low, high))
        changed = False
        for a in sorted(list(allowed)):
            allowed.remove(a)
            if a[0] > a[1] or low <= a[0] <= a[1] <= high:
                continue

            if a[0] <= low <= high <= a[1]:
                allowed.add((a[0], low - 1))
                allowed.add((high + 1, a[1]))
                changed = True
                break

            if a[0] <= low <= a[1]:
                allowed.add((a[0], low - 1))
                changed = True
                break

            if a[0] <= high <= a[1]:
                allowed.add((high + 1, a[1]))
                changed = True
                break

            allowed.add(a)

        if changed:
            lines.add((low, high))

    return sorted(list(allowed))[0][0]


def part2(data, allowed = {(0, 4294967295)}):
    """ 2016 Day 20 Part 2

    >>> part2(['5-8', '0-2', '4-7'], {(0, 9)})
    2
    """

    lines = set(tuple(int(x) for x in re.findall('\d+', line)) for line in data)

    while len(lines) > 0:
        low, high = list(lines)[0]
        lines.remove((low, high))
        changed = False
        for a in sorted(list(allowed)):
            allowed.remove(a)
            if a[0] > a[1] or low <= a[0] <= a[1] <= high:
                continue

            if a[0] <= low <= high <= a[1]:
                allowed.add((a[0], low - 1))
                allowed.add((high + 1, a[1]))
                changed = True
                break

            if a[0] <= low <= a[1]:
                allowed.add((a[0], low - 1))
                changed = True
                break

            if a[0] <= high <= a[1]:
                allowed.add((high + 1, a[1]))
                changed = True
                break

            allowed.add(a)

        if changed:
            lines.add((low, high))

    allowed = sorted(list(allowed))

    return sum([high - low + 1 for low, high in allowed])


def intersections(r1, r2):
    return [(min(r1[0], r2[0]), max(r1[0], r2[0])), (max(r1[0], r2[0]), min(r1[1], r2[1])), (min(r1[1], r2[1]), max(r1[1], r2[1]))]


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
        print(f"\nPart 1:\nLowest allowed IP address: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal allowed IP addresses: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)