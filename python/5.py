import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

from attr import has

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

import hashlib
import multiprocessing


def hash_zeroes(data, zerosStart, start, end, increment):
    word = data[:]
    for i in range(start, end, increment):
        hash_result = hashlib.md5(f'{word}{i}'.encode()).hexdigest()
        if not int(hash_result[:zerosStart], 16):
            return i, hash_result
    
    return None, None


def part1(data):
    """ 2015 Day 5 Part 1

    >>> part1(['abc'])
    '18f47a30'
    """

    p = multiprocessing.Pool()
    start_val = 0
    num_cpus = multiprocessing.cpu_count()
    inc_amt = 20000
    used = {}

    while len(used) < 8:
        for result, hash_result in p.starmap(hash_zeroes, [(data[0], 5, start_val + i * inc_amt, start_val + (i + 1) * inc_amt, 1) for i in range(num_cpus)]):
            if result and hash_result:
                used[result] = hash_result[5]

        start_val += num_cpus * inc_amt

    used = sorted(used.items(), key=lambda x: x[0])
    return ''.join([x[1] for x in used[:8]])


def part2(data):
    """ 2015 Day 5 Part 2

    >>> part2(['abc'])
    '05ace8e3'
    """

    p = multiprocessing.Pool()
    start_val = 0
    num_cpus = multiprocessing.cpu_count()
    inc_amt = 20000
    used = {str(i): (float('inf'), None) for i in range(8)}

    while any(x[1] is None for x in used.values()):
        for result, hash_result in p.starmap(hash_zeroes, [(data[0], 5, start_val + i * inc_amt, start_val + (i + 1) * inc_amt, 1) for i in range(num_cpus)]):
            if result and hash_result:
                if hash_result[5] in used and used[hash_result[5]][0] > int(result):
                    used[hash_result[5]] = (int(result), hash_result[6])

        start_val += num_cpus * inc_amt

    return ''.join([x[1][1] for x in sorted(used.items(), key=lambda x: x[0])])


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
        print(f"\nPart 1:\nPasscode: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPasscode: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)