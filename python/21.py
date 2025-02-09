import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data, password = 'abcdefgh'):
    """ 2016 Day 21 Part 1

    >>> part1(['swap position 4 with position 0', 'swap letter d with letter b', 'reverse positions 0 through 4', 'rotate left 1 step', 'move position 1 to position 4', 'move position 3 to position 0', 'rotate based on position of letter b', 'rotate based on position of letter d'], 'abcde')
    'decab'
    """

    scramble = [c for c in password]
    for line in data:
        if "swap" in line:
            if "position" in line:
                a, b = (int(x) for x in re.findall('\d+', line))
            else:
                a, b = (scramble.index(c[-1]) for c in re.findall('letter [a-z]', line))

            scramble[a], scramble[b] = scramble[b], scramble[a]
        elif "rotate" in line:
            if "right" in line:
                rot = list(int(x) for x in re.findall('\d+', line))[0]
            elif "left" in line:
                rot = -list(int(x) for x in re.findall('\d+', line))[0]
            else:
                rot = scramble.index(line[-1])
                rot += 1 if rot < 4 else 2

            rot %= len(scramble)

            scramble = scramble[-rot:] + scramble[:-rot]
        elif "reverse" in line:
            low, high = (int(x) for x in re.findall('\d+', line))
            scramble = scramble[:low] + list(reversed(scramble[low:high + 1])) + scramble[high + 1:]
        else:
            start, end = (int(x) for x in re.findall('\d+', line))
            if start > end:
                scramble = scramble[:end] + [scramble[start]] + scramble[end:start] + scramble[start + 1:]
            else:
                scramble = scramble[:start] + scramble[start + 1:end + 1] + [scramble[start]] + scramble[end + 1:]
    
    return ''.join(scramble)


def part2(data, scrambled = 'fbgdceah'):
    """ 2016 Day 21 Part 2

    >>> part2(['swap position 4 with position 0', 'swap letter d with letter b', 'reverse positions 0 through 4', 'rotate left 1 step', 'move position 1 to position 4', 'move position 3 to position 0', 'rotate based on position of letter b', 'rotate based on position of letter d'], 'decab')
    'abcde'
    """

    password = [c for c in scrambled]
    for line in data[::-1]:
        if "swap" in line:
            if "position" in line:
                a, b = (int(x) for x in re.findall('\d+', line))
            else:
                a, b = (password.index(c[-1]) for c in re.findall('letter [a-z]', line))

            password[a], password[b] = password[b], password[a]
        elif "rotate" in line:
            if "based" in line:
                i = 0
                while True:
                    scrambled = password[i:] + password[:i]
                    rot = scrambled.index(line[-1])
                    rot += 1 if rot < 4 else 2

                    rot %= len(scrambled)

                    scrambled = scrambled[-rot:] + scrambled[:-rot]

                    if scrambled == password:
                        break

                    i += 1

                password = password[i:] + password[:i]
                continue
            elif "right" in line:
                rot = list(int(x) for x in re.findall('\d+', line))[0]
            else:
                rot = -list(int(x) for x in re.findall('\d+', line))[0]

            rot *= -1
            rot %= len(password)

            password = password[-rot:] + password[:-rot]
        elif "reverse" in line:
            low, high = (int(x) for x in re.findall('\d+', line))
            password = password[:low] + list(reversed(password[low:high + 1])) + password[high + 1:]
        else:
            end, start = (int(x) for x in re.findall('\d+', line))
            if start > end:
                password = password[:end] + [password[start]] + password[end:start] + password[start + 1:]
            else:
                password = password[:start] + password[start + 1:end + 1] + [password[start]] + password[end + 1:]
    
    return ''.join(password)


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
        print(f"\nPart 1:\nScrambled: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nUnscrambled: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)