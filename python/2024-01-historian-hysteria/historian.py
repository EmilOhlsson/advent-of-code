#!/usr/bin/env python3

from argparse import ArgumentParser
from pathlib import Path
from collections import Counter

def main():
    parser = ArgumentParser(prog='historian')
    parser.add_argument('-f', '--file', nargs='?', help='Input file', type=Path, required=True)
    args = parser.parse_args()

    left = []
    right = []
    with open(args.file) as file:
        while line := file.readline():
            l,r = line.strip().split()
            print(f'Got {int(l)}, and {int(r)}')
            left.append(int(l))
            right.append(int(r))
    left.sort()
    right.sort()

    total_diff = 0
    for (l, r) in zip(left, right):
        total_diff += abs(l - r)
    print(f'Total diff: {total_diff}')

    uniqueness = 0
    counts = Counter(right)
    for value in left:
        if value in counts:
            uniqueness += value * counts[value]
    print(f'Total uniqueness: {uniqueness}')


            

if __name__ == '__main__':
    main()
