#!/usr/bin/env python3

"""Solution for Advent of code 2024-05."""

from argparse import ArgumentParser
from collections import defaultdict
from functools import cmp_to_key
from itertools import pairwise
from os import getenv
from pathlib import Path


def main():
    """Program entry point."""
    input_dir = getenv('AOC_INPUT') or '../../../inputs'
    parser = ArgumentParser(prog='ceres')
    parser.add_argument('-f', '--file', nargs='?', help='Input file',
                        type=Path, default=input_dir + '/2024/05.txt')
    args = parser.parse_args()

    configuring = True
    following: defaultdict = defaultdict(set)
    middle_page_sum_v1 = 0
    middle_page_sum_v2 = 0
    with open(args.file, encoding='utf-8') as file:
        for _, line in enumerate(file):
            line = line.strip()
            if line.strip() == "":
                configuring = False
            elif configuring:
                first, second = line.split('|')
                following[first].add(second)
            else:
                pages = line.split(',')
                in_order = True
                for first, second in pairwise(pages):
                    if second not in following[first]:
                        in_order = False
                        break
                if in_order:
                    middle_page_sum_v1 += int(pages[len(pages) // 2])
                else:
                    pages.sort(
                        key=cmp_to_key(lambda lhs, rhs:
                                       - 1 if rhs in following[lhs] else 1))
                    middle_page_sum_v2 += int(pages[len(pages) // 2])

    print(f'Sum part 1: {middle_page_sum_v1}')
    print(f'Sum part 2: {middle_page_sum_v2}')


if __name__ == '__main__':
    main()
