import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2016 Day 4 Part 1

    aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters are a (5), b (3), and then a tie between x, y, and z, which are listed alphabetically.
    a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all tied (1 of each), the first five are listed alphabetically.
    not-a-real-room-404[oarel] is a real room.
    totally-real-room-200[decoy] is not.

    >>> part1(['aaaaa-bbb-z-y-x-123[abxyz]', 'a-b-c-d-e-f-g-h-987[abcde]', 'not-a-real-room-404[oarel]', 'totally-real-room-200[decoy]'])
    1514
    """

    rooms = [Room(line) for line in data]

    sectorSums = 0
    for room in rooms:
        room.valid()
        if room.real:
            sectorSums += room.sectorID

    return sectorSums


def part2(data):
    """ 2016 Day 4 Part 2
    """

    rooms = [Room(line) for line in data]

    for room in rooms:
        room.valid()
        if room.real:
            if room.decrypt() == "northpole object storage":
                return room.sectorID
    

class keyVal:
    def __init__(self, key, val):
        self.key = key
        self.val = val

    def __lt__(self, other):
        return self.val > other.val or (self.val == other.val and self.key < other.key)
    

class Room:
    def __init__(self, roomText):
        start, end = re.search(r'\d+', roomText).span()
        self.roomName = roomText[:start].replace('-', ' ')
        self.sectorID = int(roomText[start:end])
        self.checkSum = roomText[end+1:-1]
        self.real = True

    def valid(self):
        counts = {c: len([l for l in self.roomName if l == c]) for c in self.roomName if c != ' '}
        counts = [keyVal(k, counts[k]) for k in counts.keys()]
        
        counts.sort()
        self.real = ''.join(counts[i].key for i in range(5)) == self.checkSum

    def decrypt(self):
        decrypted = ''
        for c in self.roomName:
            num = ord(c)
            if c != ' ':
                num -= ord('a')
                num += self.sectorID
                num %= 26
                num += ord('a')

            decrypted += chr(num)

        return decrypted[:-1]


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
        print(f"\nPart 1:\nSum of sector IDs of real rooms: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSector ID of North Pole Object Storage: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)