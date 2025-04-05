import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2016 Day 9 Part 1

    ADVENT contains no markers and decompresses to itself with no changes, resulting in a decompressed length of 6.
    A(1x5)BC repeats only the B a total of 5 times, becoming ABBBBBC for a decompressed length of 7.
    (3x3)XYZ becomes XYZXYZXYZ for a decompressed length of 9.
    A(2x2)BCD(2x2)EFG doubles the BC and EF, becoming ABCBCDEFEFG for a decompressed length of 11.
    (6x1)(1x3)A simply becomes (1x3)A - the (1x3) looks like a marker, but because it's within a data section of another marker, it is not treated any differently from the A that comes after it. It has a decompressed length of 6.
    X(8x2)(3x3)ABCY becomes X(3x3)ABC(3x3)ABCY (for a decompressed length of 18), because the decompressed data from the (8x2) marker (the (3x3)ABC) is skipped and not processed further.

    >>> part1(['ADVENT'])
    6
    >>> part1(['A(1x5)BC'])
    7
    >>> part1(['(3x3)XYZ'])
    9
    >>> part1(['A(2x2)BCD(2x2)EFG'])
    11
    >>> part1(['(6x1)(1x3)A'])
    6
    >>> part1(['X(8x2)(3x3)ABCY'])
    18
    """

    line = data[0]

    searchStart = 0
    marker = re.search(r'\([^(]*\)', line[searchStart:])
    while marker:
        start, end = marker.span()
        start += searchStart
        end += searchStart

        searchStart = start

        size, repeat = [int(x) for x in re.findall(r'\d+', marker.group())]
        searchStart += size * repeat
        repeat -= 1

        line = line[:start] + ''.join([line[end:end+size]] * repeat) + line[end:]

        marker = re.search(r'\([^(]*\)', line[searchStart:])

    return len(line)


def part2(data):
    """ 2016 Day 9 Part 2

    (3x3)XYZ still becomes XYZXYZXYZ, as the decompressed section contains no markers.
    X(8x2)(3x3)ABCY becomes XABCABCABCABCABCABCY, because the decompressed data from the (8x2) marker is then further decompressed, thus triggering the (3x3) marker twice for a total of six ABC sequences.
    (27x12)(20x12)(13x14)(7x10)(1x12)A decompresses into a string of A repeated 241920 times.
    (25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN becomes 445 characters long.

    >>> part2(['(3x3)XYZ'])
    9
    >>> part2(['X(8x2)(3x3)ABCY'])
    20
    >>> part2(['(27x12)(20x12)(13x14)(7x10)(1x12)A'])
    241920
    >>> part2(['(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN'])
    445

    """

    line = data[0]

    searchStart = 0
    marker = re.search(r'\([^(]*\)', line[searchStart:])
    while marker:
        start, end = marker.span()
        start += searchStart
        end += searchStart

        searchStart = start

        size, repeat = [int(x) for x in re.findall(r'\d+', marker.group())]
        searchStart += size * repeat
        repeat -= 1

        line = line[:start] + ''.join([line[end:end+size]] * repeat) + line[end:]

        marker = re.search(r'\([^(]*\)', line[searchStart:])

    return determineLen(line, {})


def determineLen(line, memo):
    length = len(line)
    marker = re.search(r'\([^(]*\)', line)
    while marker:
        start, end = marker.span()
        length -= end - start

        size, repeat = [int(x) for x in re.findall(r'\d+', marker.group())]
        repeat -= 1

        subLine = line[end:end+size]
        if subLine in memo:
            size = memo[subLine]
        else:
            size = determineLen(subLine, memo)
            memo[subLine] = size

        length += size * repeat

        line = line[:start] + line[end:]

        marker = re.search(r'\([^(]*\)', line)

    return length


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
        print(f"\nPart 1:\nLength of decompressed file: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLength of fully decompressed file: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)