from sys import stdin
input = stdin.readline


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


t = read_int()
for case_num in range(t):
    a, s = read_ints()
    s -= 2 * a
    if s < 0:
        print('No')
        continue

    valid = True
    for k in range(60, -1, -1):
        msk = 1 << k
        if (s & msk) != 0 and (a & msk) != 0:
            print('No')
            valid = False
            break

    if valid:
        print('Yes')
