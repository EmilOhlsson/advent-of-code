#!/usr/bin/env python3

"""Solution for Advent of code 2024-06."""

from argparse import ArgumentParser
from os import getenv
from pathlib import Path

Pos = tuple[int, int]
Map = dict[Pos, str]


def turn(direction):
    """Turn 90 degrees clockwise."""
    return (direction[1], -direction[0])


def step(pos, delta):
    """Add a delta to pos."""
    return (pos[0] + delta[0], pos[1] + delta[1])


def main():
    """Program entry point."""
    input_dir = getenv('AOC_INPUT') or '../../../inputs'
    parser = ArgumentParser(prog='ceres')
    parser.add_argument('-f', '--file', nargs='?', help='Input file',
                        type=Path, default=input_dir + '/2024/06.txt')
    args = parser.parse_args()

    puzzle_map: Map = {}
    with open(args.file, encoding='utf-8') as file:
        for row, line in enumerate(file):
            for col, char in enumerate(line.strip()):
                if char == '^':
                    guard_pos = (row, col)
                    char = '.'
                puzzle_map[(row, col)] = char

    visited: set[Pos] = set()
    dx: tuple[int, int] = (-1, 0)  # (row, col)

    next_pos = step(guard_pos, dx)
    while next_pos in puzzle_map:
        visited.add(guard_pos)
        # If there is an obstacle in the next position, turn
        # until path is clear
        while puzzle_map[next_pos] == '#':
            dx = turn(dx)
            next_pos = step(guard_pos, dx)

        guard_pos = step(guard_pos, dx)
        next_pos = step(guard_pos, dx)
    visited.add(guard_pos)

    print(f'Visited {len(visited)}')


if __name__ == '__main__':
    main()
