from sys import stdin
def input(): return stdin.readline().strip()


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


MOD = 998244353
n, m = read_ints()
dp = [0] * 61
dp[0] = 1
for i in range(min(n, 61)):
    ndp = [0] * 61
    for last in range(60):
        if dp[last] == 0:
            continue
        for j in range(last + 1, 61):
            if (1 << (j - 1)) > m:
                break
            if (1 << j) > m:
                choice = m - (1 << (j - 1)) + 1
            else:
                choice = 1 << (j - 1)
            ndp[j] = (ndp[j] + dp[last] * choice) % MOD
    dp = ndp

print(sum(dp) % MOD)
