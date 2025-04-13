#!/usr/bin/env python3

"""Solution for Advent of code 2024-06."""

from argparse import ArgumentParser
from collections import defaultdict
from os import getenv
from pathlib import Path

Pos = tuple[int, int]
Step = tuple[int, int]
Map = dict[Pos, str]


def rotate(direction: Step) -> Step:
    """Turn 90 degrees clockwise."""
    return (direction[1], -direction[0])


def step(pos: Pos, delta: Step) -> Pos:
    """Add a delta to pos."""
    return (pos[0] + delta[0], pos[1] + delta[1])


class PuzzleMap:
    """Solver class."""

    UP: Step = (-1, 0)

    def __init__(self, input_file):
        """Parse input file into a map and starting position."""
        self.puzzle_map: Map = {}
        with open(input_file, encoding='utf-8') as file:
            for row, line in enumerate(file):
                for col, char in enumerate(line.strip()):
                    if char == '^':
                        self.guard_pos = (row, col)

                    if char == '#':
                        self.puzzle_map[(row, col)] = char
                    else:
                        self.puzzle_map[(row, col)] = ' '

    def solve_p1(self) -> int:
        """Simply walk through map, keeping track of visited positions."""
        visited: set[Pos] = set()
        delta = self.UP  # Direction
        pos = self.guard_pos
        while pos in self.puzzle_map:
            # If there is an obstacle in the next position, turn
            # until path is clear
            pos_next = step(pos, delta)
            while self.puzzle_map.get(pos_next, '') == '#':
                delta = rotate(delta)
                pos_next = step(pos, delta)

            visited.add(pos)
            pos = pos_next

        return len(visited)

    def check_for_loop(self, pos: Pos, delta: Step) -> bool:
        """Check if start traveling here would result in loop."""
        visited: defaultdict[Pos, set[Pos]] = defaultdict(set)
        while pos in self.puzzle_map:
            # Check if next position has an obstacle, if so: turn
            pos_next = step(pos, delta)
            rotations = 0
            while self.puzzle_map.get(pos_next, '') == '#' and rotations < 4:
                delta = rotate(delta)
                pos_next = step(pos, delta)
                rotations += 1
            if rotations == 4:
                return True

            # Check if we've already been here, traveling in same direction
            if delta in visited[pos]:
                return True

            # Consider this position visited
            visited[pos].add(delta)
            pos = pos_next

        return False

    def solve_p2(self):
        """Walk through map, and in each point consider possible blockade."""
        pos = self.guard_pos
        visited: defaultdict[Pos, set[Pos]] = defaultdict(set)
        delta = self.UP
        loop_points: set[Pos] = set()

        while pos in self.puzzle_map:
            # If there is an obstacle in the next position, turn
            # until path is clear
            pos_next = step(pos, delta)
            while self.puzzle_map.get(pos_next, '') == '#':
                delta = rotate(delta)
                pos_next = step(pos, delta)

            # What would happen if we place an obstacle in front of us. No need
            # to attempt to place obstacle if we've already passed the candidate
            # position of pos_next
            if pos_next in self.puzzle_map and len(visited[pos_next]) == 0:
                assert self.puzzle_map[pos_next] != '#'

                if rotate(delta) in visited[pos]:
                    # No need to do full check
                    loop_points.add(pos_next)
                else:
                    self.puzzle_map[pos_next] = '#'
                    if self.check_for_loop(pos, rotate(delta)):
                        loop_points.add(pos_next)
                    self.puzzle_map[pos_next] = ' '

            # Mark current position as visited, and logging direction
            visited[pos].add(delta)
            pos = pos_next

        return len(loop_points)


def main():
    """Program entry point."""
    input_dir = getenv('AOC_INPUT') or '../../../inputs'
    parser = ArgumentParser(prog='ceres')
    parser.add_argument('-f', '--file', nargs='?', help='Input file',
                        type=Path, default=input_dir + '/2024/06.txt')
    args = parser.parse_args()
    puzzle_map = PuzzleMap(args.file)

    print(f'Visited {puzzle_map.solve_p1()}')      # 5030
    print(f'Loop points {puzzle_map.solve_p2()}')  # 278, 422 is too low, 4484 is too high


if __name__ == '__main__':
    main()
