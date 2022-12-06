import sys

line = sys.stdin.readline()
for i in range(len(line)):
    s = set(line[i:i+14])
    if len(s) >= 14:
        print(i+14)
        exit(0)

