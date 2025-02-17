import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict
import re


def part1(data, compare = [17, 61]):
    """ 2016 Day 10 Part 1

    >>> part1(['value 5 goes to bot 2', 'bot 2 gives low to bot 1 and high to bot 0', 'value 3 goes to bot 1', 'bot 1 gives low to output 1 and high to bot 0', 'bot 0 gives low to output 2 and high to output 0', 'value 2 goes to bot 2'], [2, 5])
    2
    """

    bots = defaultdict(lambda: [])

    ranLines = [False] * len(data)
    while not all(ranLines):
        for i, line in enumerate(data):
            if ranLines[i]:
                continue

            line = line.split(' ')
            vals = []
            for l in line:
                if re.search(r'\d+', l):
                    vals.append(int(l))

            if len(vals) == 2:
                bots[f'bot {vals[1]}'].append(vals[0])
                bots[f'bot {vals[1]}'].sort()
                ranLines[i] = True
            elif len(bots[f'bot {vals[0]}']) == 2:
                if bots[f'bot {vals[0]}'] == compare:
                    return vals[0]
                
                for pi, b in zip([5, 10], vals[1:]):
                    bots[f'{line[pi]} {b}'].append(bots[f'bot {vals[0]}'].pop(0))
                    bots[f'{line[pi]} {b}'].sort()

                ranLines[i] = True

    return -1


def part2(data):
    """ 2016 Day 10 Part 2
    """

    bots = defaultdict(lambda: [])

    ranLines = [False] * len(data)
    while not all(ranLines):
        for i, line in enumerate(data):
            if ranLines[i]:
                continue

            line = line.split(' ')
            vals = []
            for l in line:
                if re.search(r'\d+', l):
                    vals.append(int(l))

            if len(vals) == 2:
                bots[f'bot {vals[1]}'].append(vals[0])
                bots[f'bot {vals[1]}'].sort()
                ranLines[i] = True
            elif len(bots[f'bot {vals[0]}']) == 2:
                if bots[f'bot {vals[0]}'] == [17, 61]:
                    part1 = vals[0]

                for pi, b in zip([5, 10], vals[1:]):
                    bots[f'{line[pi]} {b}'].append(bots[f'bot {vals[0]}'].pop(0))
                    bots[f'{line[pi]} {b}'].sort()

                ranLines[i] = True

    return bots['output 0'][0] * bots['output 1'][0] * bots['output 2'][0]


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
        print(f"\nPart 1:\nBot that compares value 17 and 61 microchips: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nProduct of outputs 0-2: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)