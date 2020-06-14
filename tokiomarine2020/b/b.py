def read_int():
    return int(input())


def read_ints():
    return map(int, input().split(' '))


a, v = read_ints()
b, w = read_ints()
t = read_int()
if w >= v:
    print('NO')
elif abs(a - b) / (v - w) <= t:
    print('YES')
else:
    print('NO')
