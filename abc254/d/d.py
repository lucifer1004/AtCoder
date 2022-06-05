from sys import stdin
def input(): return stdin.readline().strip()


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


n = read_int()
primes = []
is_prime = [False] * 2 + [True] * (n - 1)
for i in range(2, n + 1):
    if is_prime[i]:
        primes.append(i)
    for p in primes:
        if p * i > n:
            break
        is_prime[p * i] = False
        if i % p == 0:
            break

sqrt = [1] * (n + 1)
for i in range(2, n + 1):
    sqrt[i] = sqrt[i - 1]
    if (sqrt[i] + 1) * (sqrt[i] + 1) <= i:
        sqrt[i] += 1


ans = 0
for i in range(1, n + 1):
    must = 1
    for p in primes:
        if p * p > i:
            break
        if i % p == 0:
            cnt = 0
            while i % p == 0:
                i //= p
                cnt += 1
            if cnt % 2 == 1:
                must *= p
    if i > 1:
        must *= i
    ans += sqrt[n // must]

print(ans)
