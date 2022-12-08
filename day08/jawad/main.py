#!/usr/bin/env python3
import sys

N = 99

heights = [[]] * N

for i,line in enumerate(sys.stdin):
    heights[i] = [int(h) for h in line.rstrip()]

def getscore(x,h):
    s = 0
    for t in x:
        s += 1
        if t >= h: break
    return s

count = 0
max_scenic = 0

for r in range(1,N-1):
    for c in range(1,N-1):
        left = list(reversed(heights[r][:c]))
        right = heights[r][c+1:]
        top = list(reversed([v[c] for v in heights][:r]))
        bot = [v[c] for v in heights][r+1:]

        count += 1 if heights[r][c] > min([max(left),max(right),max(top),max(bot)]) else 0

        scores = [getscore(a,heights[r][c]) for a in [left,right,top,bot]]
        score = scores[0] * scores[1] * scores[2] * scores[3]
        max_scenic = max([max_scenic,score])

print(count + 4*N - 4)
print(max_scenic)

