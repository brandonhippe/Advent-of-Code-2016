import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2016 Day 25 Part 1
    """

    instructions = [line.split(' ') for line in data]

    a = 0
    regs = defaultdict(lambda: 0)
    regs['printed'] = []
    regs['a'] = a
    while 0 <= regs['PC'] < 8:
        op, *text = instructions[regs['PC']]
        OPS[op](regs, text)
        regs['PC'] += 1

    while not alternatingBits(regs['d']):
        a += 1
        regs['d'] += 1

    return a


def part2(data):
    """ 2016 Day 25 Part 2
    """

    return "Christmas has been saved!"


def cpy(regs, text):
    x, y = text
    if len(re.findall('-?\d+', x)) != 0:
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


def out(regs, x):
    x = x[0]
    if len(re.findall('-?\d+', x)) != 0:
        x = int(x)
    else:
        x = regs[x]

    regs['printed'].append(x)


OPS = {"cpy": cpy, "inc": inc, "dec": dec, "jnz": jnz, "out": out}


def alternatingBits(n):
    n = bin(n)[2:]
    for i in range(1, len(n)):
        if n[i] == n[i - 1]:
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
        print(f"\nPart 1:\nLowest value to initialize register A: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n{p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)