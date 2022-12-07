#!/usr/bin/env python3
import sys

stack = []
filemap = {'/': []}
sizes = {'/':0}

def pwd():
    path = ''
    for d in stack:
        path += '/'
        path += d
    if path == '':
        return '/'
    return path

for line in sys.stdin:
    cmd = line.rstrip().split(' ')
    if cmd[0] == '$':
        if cmd[1] == 'cd':
            if cmd[2] == '..':
                stack.pop()
            elif cmd[2] == '/':
                stack = []
            else:
                stack.append(cmd[2])
                path = pwd()
                if path not in filemap:
                    filemap[path] = []
                    sizes[path] = 0
    elif cmd[0].isdigit():
        path = pwd()
        size = int(cmd[0])
        file = cmd[1]
        if file not in filemap[path]:
            filemap[path].append(file)
            backtrack = path[1:].split('/')
            if backtrack[0] == '':
                sizes['/'] += size
            else:
                for i in range(1,len(backtrack)+1):
                    sizes['/'+'/'.join(backtrack[:i])] += size
                sizes['/'] += size

sizes = sorted(sizes.values())

print(sum([s for s in sizes if s <= 100000]))
to_free = sizes[-1] - 40000000
print(min([s for s in sizes if s >= to_free]))

