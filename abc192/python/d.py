x = input()
n = len(x)
m = int(input())
d = max(map(int, list(x)))
lo = d + 1
hi = m

if hi < d:
    print(0)
    exit(0)

if n == 1:
    print(1)
    exit(0)

while lo <= hi:
    mid = (lo + hi) // 2
    base = 1
    acc = 0
    for i in range(n):
        acc += int(x[n - 1 - i]) * base
        base *= mid
    if acc <= m:
        lo = mid + 1
    else:
        hi = mid - 1
print(hi - d)
