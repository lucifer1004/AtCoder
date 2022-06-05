from sys import stdin
def input(): return stdin.readline().strip()


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


n, k = read_ints()
a = list(read_ints())
s = [0] * n
for i in range(k):
    f = sorted(a[i::k])
    for j, fi in enumerate(f):
        s[i + j * k] = fi
if s == sorted(a):
    print('Yes')
else:
    print('No')
