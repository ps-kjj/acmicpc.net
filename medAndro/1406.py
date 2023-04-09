import sys
from collections import deque

leftDeque = deque(sys.stdin.readline().rstrip())
rightDeque = deque()

for i in range(int(sys.stdin.readline().rstrip())):
    cmd = sys.stdin.readline().rstrip()
    if cmd == 'L' and leftDeque:
        rightDeque.appendleft(leftDeque.pop())
    if cmd == 'D' and rightDeque:
        leftDeque.append(rightDeque.popleft())
    if cmd == 'B' and leftDeque:
        leftDeque.pop()
    elif cmd.startswith('P '):
        leftDeque.append(cmd[2:])

sys.stdout.write(''.join(leftDeque))
sys.stdout.write(''.join(rightDeque))