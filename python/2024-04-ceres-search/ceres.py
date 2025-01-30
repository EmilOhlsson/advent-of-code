#!/usr/bin/env python3

from argparse import ArgumentParser
from collections import defaultdict
from pathlib import Path
from typeguard import typechecked

postype = tuple[int,int]
containertype = defaultdict[postype, str]

@typechecked
def xmas_at(pos: postype, ch: containertype) -> int:
    r,c = pos

    candidates = [
            f'{ch[(r+0,c+0)]}{ch[(r+0,c+1)]}{ch[(r+0,c+2)]}{ch[(r+0,c+3)]}',
            f'{ch[(r+0,c+0)]}{ch[(r+1,c+0)]}{ch[(r+2,c+0)]}{ch[(r+3,c+0)]}',
            f'{ch[(r+0,c+0)]}{ch[(r+1,c+1)]}{ch[(r+2,c+2)]}{ch[(r+3,c+3)]}',
            f'{ch[(r+0,c+0)]}{ch[(r+1,c-1)]}{ch[(r+2,c-2)]}{ch[(r+3,c-3)]}', ]

    xmas = { 'XMAS', 'SAMX' }
    return sum([c in xmas for c in candidates])


def main():
    parser = ArgumentParser(prog='ceres')
    parser.add_argument('-f', '--file', nargs='?', help='Input file',
                        type=Path, default='../../inputs/2024/04.txt')
    args = parser.parse_args()

    chars: containertype = defaultdict(lambda: '.')
    with open(args.file) as file:
        for row, line in enumerate(file):
            chars.update({ (row, col): ch for col, ch in
                          enumerate(line.strip()) })

    count: int = 0
    key_set = set(chars.keys())
    for pos in key_set:
        count += xmas_at(pos, chars)
    
    print(f'Counted {count} XMAS')


if __name__ == '__main__':
    main()
