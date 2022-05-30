from sys import stdin
def input(): return stdin.readline().strip()


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


n = read_int()
a = [0] + list(read_ints())
b = [0] + list(read_ints())
ma = [0] * (n * 2 + 1)
mb = [0] * (n * 2 + 1)
for i in range(1, n * 2 + 1):
    ma[a[i]] = i
    mb[b[i]] = i
ans = [''] * (n * 2 + 1)
sa = [''] * (n * 2 + 1)
sb = [''] * (n * 2 + 1)


def valid(s):
    depth = 0
    for ch in s:
        if ch == '(':
            depth += 1
        else:
            depth -= 1
        if depth < 0:
            return False
    return depth == 0


def issorted(a):
    for pre, nxt in zip(a[:-1], a[1:]):
        if pre > nxt:
            return False
    return True


def noanswer():
    print(-1)
    exit()


pa = ma[b[n * 2]]
pb = mb[a[1]]
if pa == 1 or pb == n * 2:
    noanswer()

ans = [''] * (n * 2 + 1)
la = 1
rb = 2 * n

while True:
    npa, npb = pa, pb
    for i in range(la, pa):
        if ans[a[i]] == '':
            ans[a[i]] = '('
            npb = min(npb, mb[a[i]])
    for i in range(pb + 1, rb + 1):
        if ans[b[i]] == '':
            ans[b[i]] = ')'
            npa = max(npa, ma[b[i]])
    while la < n * 2 and ans[a[la]] != '':
        la += 1
    while rb > 0 and ans[b[rb]] != '':
        rb -= 1
    if npa == pa and npb == pb:
        if npa < 2 * n and ans[a[npa + 1]] == '':
            ans[a[npa + 1]] = '('
            npb = min(npb, mb[a[npa + 1]])
        elif npb > 1 and ans[b[npb - 1]] == '':
            ans[b[npb - 1]] = ')'
            npa = max(npa, ma[b[npb - 1]])
        else:
            break
    pa, pb = npa, npb

sa = ''.join(ans[a[i]] for i in range(1, n * 2 + 1))
sb = ''.join(ans[b[i]] for i in range(1, n * 2 + 1))

if not valid(sa) or not valid(sb):
    noanswer()

la = [a[i] for i in range(1, n * 2 + 1) if ans[a[i]] == '(']
ra = [a[i] for i in range(1, n * 2 + 1) if ans[a[i]] == ')']
lb = [b[i] for i in range(1, n * 2 + 1) if ans[b[i]] == '(']
rb = [b[i] for i in range(1, n * 2 + 1) if ans[b[i]] == ')']

if not issorted(la) or not issorted(ra):
    noanswer()

if lb[::-1] != la or rb[::-1] != ra:
    noanswer()

print(''.join(ans[1:]))
