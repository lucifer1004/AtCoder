from sys import stdin
def input(): return stdin.readline().strip()


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


n = read_int()
c = [[1] * n for _ in range(n)]
for i in range(2, n):
    for j in range(1, i):
        c[i][j] = c[i - 1][j - 1] + c[i - 1][j]

for i in range(n):
    print(' '.join(map(str, c[i][:i + 1])))
