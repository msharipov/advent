#!/usr/bin/env python3
with open("input.txt") as file:
    total = 0
    for line in file.readlines():
        sides = [int(side) for side in line.split('x')]
        sides.sort()
        total += 3 * sides[0] * sides[1]
        total += 2 * sides[0] * sides[2]
        total += 2 * sides[1] * sides[2]
    print(total)
