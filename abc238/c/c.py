from sys import stdin
input = stdin.readline


def read_int():
    return int(input())


n = read_int()
lower = 1
upper = 10
ans = 0
while n >= upper:
    r = upper - lower
    ans += r * (r + 1) // 2
    lower *= 10
    upper *= 10

r = n - lower + 1
ans += r * (r + 1) // 2

print(ans % 998244353)
