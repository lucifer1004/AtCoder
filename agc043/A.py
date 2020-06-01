def read_int():
    return int(input())


def read_ints():
    return map(int, input().split(' '))


r, c = read_ints()
s = []
for i in range(r):
    s.append(input())
inf = int(1e9)
dp = [[inf for j in range(c)] for i in range(r)]
dp[0][0] = 1 if s[0][0] == '#' else 0
for i in range(r):
    for j in range(c):
        if i > 0:
            dp[i][j] = dp[i - 1][j]
            if s[i][j] == '#' and s[i - 1][j] == '.':
                dp[i][j] += 1
        if j > 0:
            another = dp[i][j - 1]
            if s[i][j] == '#' and s[i][j - 1] == '.':
                another += 1
            dp[i][j] = min(dp[i][j], another)
print(dp[r - 1][c - 1])
