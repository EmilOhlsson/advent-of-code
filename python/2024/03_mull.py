#!/usr/bin/env python3

from argparse import ArgumentParser
from pathlib import Path
import re

def main():
    parser = ArgumentParser(prog='mull')
    parser.add_argument('-f', '--file', nargs='?', help='Input file', type=Path, required=True)
    args = parser.parse_args()

    pattern = re.compile(r"""mul\((\d{1,3}),(\d{1,3})\)""")
    pattern_v2 = re.compile(r"""mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))""")
    total_sum_v1 = 0
    total_sum_v2 = 0
    enable = True
    with open(args.file) as file:
        while line := file.readline():
            matches = pattern.findall(line)
            for a, b in matches:
                total_sum_v1 += int(a) * int(b)

            matches = pattern_v2.findall(line)
            for a, b, do, dont in matches:
                if do == 'do()':
                    enable = True
                elif dont == "don't()":
                    enable = False
                elif enable:
                    total_sum_v2 += int(a) * int(b)
    print(f'Total sum: {total_sum_v1}')
    print(f'Total sum: {total_sum_v2}')


if __name__ == '__main__':
    main()
