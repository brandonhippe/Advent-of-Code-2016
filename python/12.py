import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict
from functools import cache


def part1(data):
    """ 2016 Day 12 Part 1

    >>> part1(['cpy 41 a', 'inc a', 'inc a', 'dec a', 'jnz a 2', 'dec a'])
    42
    """

    instructions = [line.split(' ') for line in data]

    regs = defaultdict(lambda: 0)
    while 0 <= regs['PC'] < min(9, len(instructions)):
        op, *text = instructions[regs['PC']]
        OPS[op](regs, text)
        regs['PC'] += 1

    return calc(regs, [int(re.findall(r'-?\d+', l[1])[0]) for l in instructions[16:18]]) if regs['PC'] < len(instructions) else regs['a']


def part2(data):
    """ 2016 Day 12 Part 2
    """

    instructions = [line.split(' ') for line in data]

    regs = defaultdict(lambda: 0)
    regs['c'] = 1
    while 0 <= regs['PC'] < min(9, len(instructions)):
        op, *text = instructions[regs['PC']]
        OPS[op](regs, text)
        regs['PC'] += 1

    return calc(regs, [int(re.findall(r'-?\d+', l[1])[0]) for l in instructions[16:18]])


def cpy(regs, text):
    x, y = text
    if len(re.findall(r'-?\d+', x)) != 0:
        x = int(x)
    else:
        x = regs[x]

    regs[y] = x


def inc(regs, x):
    regs[x[0]] += 1


def dec(regs, x):
    regs[x[0]] -= 1


def jnz(regs, text):
    x, y = text
    y = int(y)
    if len(re.findall(r'-?\d+', x)) != 0:
        x = int(x)
    else:
        x = regs[x]

    if x != 0:
        regs['PC'] += y - 1


OPS = {"cpy": cpy, "inc": inc, "dec": dec, "jnz": jnz}


@cache
def fib(n):    
    if n <= 1:
        return 1

    return fib(n - 1) + fib(n - 2)


def calc(regs, nums):
    regs['a'] = fib(regs['d'] + 1)

    return regs['a'] + nums[0] * nums[1]


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
        print(f"\nPart 1:\nValue in register a after program executes: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nValue in register a after program executes: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)