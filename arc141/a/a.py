from sys import stdin
def input(): return stdin.readline().strip()


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


t = read_int()
for case_num in range(t):
    n = read_int()
    s = str(n)
    m = len(s)
    ans = 0

    for i in range(1, m // 2 + 1):
        if m % i != 0:
            continue
        parts = [s[j * i:(j + 1) * i] for j in range(m // i)]
        a1 = int(parts[0] * (m // i))
        if a1 <= n:
            ans = max(ans, a1)
        if len(str(int(parts[0]) - 1)) == i:
            ans = max(ans, int(str(int(parts[0]) - 1) * (m // i)))

    if ans == 0:
        ans = int('9' * (m - 1))
    print(ans)
