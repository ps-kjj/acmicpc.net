import sys
input = sys.stdin.readline
print = sys.stdout.write

n = int(input().rstrip())
stack = []
len = 0
for i in range(n):
    cmd= input().rstrip()
    if "push" in cmd:
        stack.append(cmd.split()[1])
        len +=1
    elif "pop" == cmd:
        if len == 0:
            print("%s\n" % -1)
        else:
            print("%s\n" % stack.pop())
            len -= 1
    elif "size" == cmd:
        print("%s\n" % len)
    elif "empty" == cmd:
        if len == 0:
            print("%s\n" % 1)
        else:
            print("%s\n" % 0)
    else:
        if len == 0:
            print("%s\n" % -1)
        else:
            print("%s\n" % stack[len-1])