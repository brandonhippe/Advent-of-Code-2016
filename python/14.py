import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import hashlib


def part1(data):
    """ 2016 Day 14 Part 1

    >>> part1(['abc'])
    22728
    """

    word = data[0]

    i = 0
    keys = []
    posKeys = []
    trips = {}
    while len([k for k in keys if trips[k][1]]) < 64:
        for k in keys[::-1]:
            if k + 1000 < i and not trips[k][1]:
                keys.pop(keys.index(k))

        for k in posKeys[::-1]:
            if k + 1000 < i and not trips[k][1]:
                posKeys.pop(posKeys.index(k))
        
        while len(keys) < 64 and len(posKeys) != 0:
                keys.append(posKeys.pop(0))

        result = hashlib.md5(f'{word}{i}'.encode()).hexdigest()

        quint = quintuple(result)
        if len(quint) != 0:
            for pos, (char, used) in zip(trips.keys(), trips.values()):
                if not used:
                    if char in quint:
                        trips[pos][1] = True

        trip = triple(result)
        if len(trip) == 1:
            posKeys.append(i)
            trips[i] = [trip, False]

        i += 1

    return keys[63]


def part2(data):
    """ 2016 Day 14 Part 2

    >>> part2(['abc'])
    22551
    """

    word = data[0]

    i = 0
    keys = []
    posKeys = []
    trips = {}
    while len([k for k in keys if trips[k][1]]) < 64:
        for k in keys[::-1]:
            if k + 1000 < i and not trips[k][1]:
                keys.pop(keys.index(k))

        for k in posKeys[::-1]:
            if k + 1000 < i and not trips[k][1]:
                posKeys.pop(posKeys.index(k))
        
        while len(keys) < 64 and len(posKeys) != 0:
                keys.append(posKeys.pop(0))

        result = stretchHash(word, i)

        quint = quintuple(result)
        if len(quint) != 0:
            for pos, (char, used) in zip(trips.keys(), trips.values()):
                if not used:
                    if char in quint:
                        trips[pos][1] = True

        trip = triple(result)
        if len(trip) == 1:
            posKeys.append(i)
            trips[i] = [trip, False]

        i += 1

    return keys[63]


def triple(h):
    for i in range(len(h) - 2):
        if len(set(h[i:i + 3])) == 1:
            return h[i]

    return ''


def quintuple(h):
    groups = ''
    for i in range(len(h) - 4):
        if len(set(h[i:i + 5])) == 1:
            groups += h[i]

    return groups


def stretchHash(salt, ix):
    result = hashlib.md5(f'{salt}{ix}'.encode()).hexdigest()
    for _ in range(2016):
        result = hashlib.md5(f'{result}'.encode()).hexdigest()

    return result


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
        print(f"\nPart 1:\nIndex that produces 64th key: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nIndex that produces 64th key: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)