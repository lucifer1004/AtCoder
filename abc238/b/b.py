from sys import stdin
input = stdin.readline


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


n = read_int()
a = list(read_ints())
s = set()
s.add(0)
angle = 0
for ai in a:
    angle += ai
    angle %= 360
    s.add(angle)
v = sorted(list(s))
v.append(v[0] + 360)
print(max(y - x for x, y in zip(v[:-1], v[1:])))
