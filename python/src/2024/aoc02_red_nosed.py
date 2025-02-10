#!/usr/bin/env python3

from argparse import ArgumentParser
from pathlib import Path
from itertools import islice
from collections import deque


def sliding_window(iterable, n):
    iterator = iter(iterable)
    window = deque(islice(iterator, n -1), maxlen=n)
    for x in iterator:
        window.append(x)
        yield tuple(window)


def quantify(iterable, predicate=bool):
    return sum(map(predicate, iterable))


def is_safe_v1(levels) -> bool:
    if all(levels[i] < levels[i+1] for i in range(len(levels) - 1)):
        if all(levels[i+1] - levels[i] <= 3 for i in range(len(levels) - 1)):
            return True
    elif all(levels[i] > levels[i+1] for i in range(len(levels) - 1)):
        if all(levels[i] - levels[i + 1] <= 3 for i in range(len(levels) - 1)):
            return True
    return False


def is_safe_v2(levels) -> bool:
    if is_safe_v1(levels):
        return True

    for skip_i in range(len(levels)):
        new_list = [v for i, v in enumerate(levels) if i != skip_i]
        if is_safe_v1(new_list):
            return True
    return False
    

def main():
    parser = ArgumentParser(prog='red-nosed')
    parser.add_argument('-f', '--file', nargs='?', help='Input file', type=Path, required=True)
    args = parser.parse_args()

    safe_v1 = 0
    safe_v2 = 0
    with open(args.file) as file:
        while line := file.readline():
            levels = [ int(val) for val in line.strip().split() ]
            if is_safe_v1(levels):
                safe_v1 += 1
            if is_safe_v2(levels):
                safe_v2 += 1

    print(f'Safe v1: {safe_v1}')
    print(f'Safe v2: {safe_v2}')


            

if __name__ == '__main__':
    main()
