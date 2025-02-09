import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2016 Day 23 Part 1

    >>> part1(['cpy 2 a', 'tgl a', 'tgl a', 'tgl a', 'cpy 1 a', 'dec a', 'dec a'])
    3
    """

    instructions = [line.split(' ') for line in data]

    regs = defaultdict(lambda: 0)
    regs['a'] = 7

    while 0 <= regs['PC'] < len(instructions):
        op, *text = instructions[regs['PC']]
        OPS[op](regs, instructions, text)
        regs['PC'] += 1

    return regs['a']


def part2(data):
    """ 2016 Day 23 Part 2
    """

    instructions = [line.split(' ') for line in data]

    regs = defaultdict(lambda: 0)
    regs['a'] = 12

    while 0 <= regs['PC'] < len(instructions):
        if regs['PC'] == 4:
            regs['a'] += regs['b'] * regs['d']
            regs['b'] -= 1
            regs['c'] = 2 * regs['b']
            regs['d'] = 0
            regs['PC'] = 16
        else:
            op, *text = instructions[regs['PC']]
            OPS[op](regs, instructions, text)
            regs['PC'] += 1

    return regs['a']


def cpy(regs, _, text):
    x, y = text
    if len(re.findall('-?\d+', x)) != 0:
        x = int(x)
    else:
        x = regs[x]

    regs[y] = x


def inc(regs, _, x):
    regs[x[0]] += 1


def dec(regs, _, x):
    regs[x[0]] -= 1


def jnz(regs, _, text):
    x, y = text
    
    if len(re.findall('-?\d+', x)) != 0:
        x = int(x)
    else:
        x = regs[x]

    if len(re.findall('-?\d+', y)) != 0:
        y = int(y)
    else:
        y = regs[y]

    if x != 0:
        regs['PC'] += y - 1


TOGGLES = {"cpy": "jnz", "inc": "dec", "dec": "inc", "jnz": "cpy", "tgl": "inc"}


def tgl(regs, instructions, x):
    x = x[0]
    if len(re.findall('-?\d+', x)) != 0:
        x = int(x)
    else:
        x = regs[x]

    ix = regs['PC'] + x
    if 0 <= ix < len(instructions):
        instructions[ix][0] = TOGGLES[instructions[ix][0]]


OPS = {"cpy": cpy, "inc": inc, "dec": dec, "jnz": jnz, "tgl": tgl}


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
        print(f"\nPart 1:\nValue sent to safe: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nValue sent to safe: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)