from sys import stdin
int(stdin.readline())
nSet = set(stdin.readline().split())
int(stdin.readline())
mTuple = tuple(stdin.readline().split())

for i in mTuple:
    if i in nSet:
        print(1, end=" ")
    else:
        print(0, end=" ")