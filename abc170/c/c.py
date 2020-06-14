def read_int():
    return int(input())


def read_ints():
    return map(int, input().split(' '))


x, n = read_ints()
p = []
if n > 0:
    p = list(read_ints())
ans = -1
delta = 1000
for i in range(-1, 102):
    ok = True
    for j in p:
        if j == i:
            ok = False
            break
    if ok:
        d = abs(x - i)
        if d < delta:
            ans = i
            delta = d
print(ans)
