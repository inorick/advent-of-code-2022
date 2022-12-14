#!/usr/bin/env python3
import sys

x = 1
xvalues = []

for line in sys.stdin:
    xvalues.append(x)
    line = line.rstrip()
    if line == 'noop':
        continue
    addval = int(line.split()[1])
    xvalues.append(x)
    x += addval

sum = 0
for i in range(19,len(xvalues),40):
    sum += (i + 1) * xvalues[i]
print(sum)

