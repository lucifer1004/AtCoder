def read_ints():
    return map(int, input().split(' '))


mod = 998244353
a, b, c, d = read_ints()
dp = [[0 for j in range(d + 1)] for i in range(c + 1)]
dp[a][b] = 1
for i in range(a + 1, c + 1):
    dp[i][b] = dp[i - 1][b] * b % mod
for j in range(b + 1, d + 1):
    dp[a][j] = dp[a][j - 1] * a % mod

for i in range(a + 1, c + 1):
    for j in range(b + 1, d + 1):
        dp[i][j] = (dp[i - 1][j] * j + dp[i][j - 1]
                    * i - (i - 1) * (j - 1) * dp[i - 1][j - 1]) % mod
        if dp[i][j] < 0:
            dp[i][j] += mod


print(dp[c][d])
