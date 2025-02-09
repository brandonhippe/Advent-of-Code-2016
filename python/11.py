import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re



def part1(data):
    """ 2016 Day 11 Part 1

    *** Example doesn't work for this solution, but actual inputs work
    """

    return steps([len(re.findall(' a ', line)) for line in data])


def part2(data):
    """ 2016 Day 11 Part 1
    """

    counts = [len(re.findall(' a ', line)) for line in data]

    return steps([counts[0] + 4, *counts[1:]])


def steps(counts):
    steps = 0
    while counts[-1] != sum(counts):
        for i, n in enumerate(counts):
            if n != 0:
                counts[i] = 0
                counts[i + 1] += n
                steps += 2 * (n-1) - 1
                break
    
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
        print(f"\nPart 1:\nSteps to get all components on 4th floor: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSteps to get all components on 4th floor: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)