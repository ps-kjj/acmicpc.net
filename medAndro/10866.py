import sys
from collections import deque

mydeque = deque()

def parse_txt(str):
    if str[0] == 'push_front':
        mydeque.appendleft(str[1])
    elif str[0] == 'push_back':
        mydeque.append(str[1])
    elif str[0] == 'pop_front':
        if mydeque:
            print(mydeque.popleft())
        else:
            print(-1)
    elif str[0] == 'pop_back':
        if mydeque:
            print(mydeque.pop())
        else:
            print(-1)
    elif str[0] == 'size':
        print(len(mydeque))
    elif str[0] == 'empty':
        print(int(not bool(mydeque)))
    elif str[0] == 'front':
        if mydeque:
            print(mydeque[0])
        else:
            print(-1)
    elif str[0] == 'back':
        if mydeque:
            print(mydeque[-1])
        else:
            print(-1)

num = int(sys.stdin.readline().rstrip())

for i in range(num):
    parse_txt(sys.stdin.readline().split())