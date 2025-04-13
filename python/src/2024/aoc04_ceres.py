#!/usr/bin/env python3

"""Solution for Advent of code 2024-04."""

from argparse import ArgumentParser
from collections import defaultdict
from os import getenv
from pathlib import Path

from typeguard import typechecked

Pos = tuple[int, int]
Map = defaultdict[Pos, str]


@typechecked
def xmas_at(pos: Pos, ch: Map) -> int:
    """Count number of possible XMAS at `pos`."""
    r, c = pos

    candidates = [
        f'{ch[(r+0, c+0)]}{ch[(r+0, c+1)]}{ch[(r+0, c+2)]}{ch[(r+0, c+3)]}',
        f'{ch[(r+0, c+0)]}{ch[(r+1, c+0)]}{ch[(r+2, c+0)]}{ch[(r+3, c+0)]}',
        f'{ch[(r+0, c+0)]}{ch[(r+1, c+1)]}{ch[(r+2, c+2)]}{ch[(r+3, c+3)]}',
        f'{ch[(r+0, c+0)]}{ch[(r+1, c-1)]}{ch[(r+2, c-2)]}{ch[(r+3, c-3)]}', ]

    xmas = {'XMAS', 'SAMX'}
    return sum(c in xmas for c in candidates)


@typechecked
def cross_mas_at(pos: Pos, ch: Map) -> int:
    """Check if there is a cross-mas at `pos`."""
    r, c = pos

    candidates = [
            f'{ch[(r-1, c-1)]}{ch[pos]}{ch[(r+1, c+1)]}',
            f'{ch[(r+1, c-1)]}{ch[pos]}{ch[(r-1, c+1)]}', ]
    xmas = {'MAS', 'SAM'}
    return 1 if all(cnd in xmas for cnd in candidates) else 0


def main():
    """Program entry point."""
    input_dir = getenv('AOC_INPUT') or '../../inputs'
    parser = ArgumentParser(prog='ceres')
    parser.add_argument('-f', '--file', nargs='?', help='Input file',
                        type=Path, default=input_dir + '/2024/04.txt')
    args = parser.parse_args()

    chars: Map = defaultdict(lambda: '.')
    with open(args.file, encoding='utf-8') as file:
        for row, line in enumerate(file):
            chars.update({(row, col): ch for col, ch in
                          enumerate(line.strip())})

    count_v1: int = 0
    count_v2: int = 0
    key_set = set(chars.keys())
    for pos in key_set:
        count_v1 += xmas_at(pos, chars)
        count_v2 += cross_mas_at(pos, chars)

    print(f'Counted {count_v1} XMAS')
    print(f'Counted {count_v2} CROSSMAS')


if __name__ == '__main__':
    main()
