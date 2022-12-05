#!/usr/bin/env python3

import sys

NSTACKS = 9
PART1 = False

stacks = [[] for _ in range(NSTACKS)]

def line_to_crates(line):
    return [c for c in line[1::4]]

for _ in range(NSTACKS):
    for i,c in enumerate(line_to_crates(sys.stdin.readline())):
        if c != ' ':
            stacks[i].append(c)

for s in stacks:
    s.reverse()

# discard stack number line
sys.stdin.readline()

for line in sys.stdin:
    if line[0] == '\n': continue
    n,frm,to = [int(line.split(' ')[i]) for i in [1,3,5]]
    frm -= 1
    to -= 1
    if PART1:
        stacks[to] += list(reversed(stacks[frm][-n:]))
    else:
        stacks[to] += stacks[frm][-n:]
    stacks[frm] = stacks[frm][:-n]

print(''.join(list(map(lambda x: x[-1], stacks))))

