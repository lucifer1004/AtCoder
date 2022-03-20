from sys import stdin
from collections import deque
def input(): return stdin.readline().strip()


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


n, m = read_ints()
adj = [[] for _ in range(n)]
for _ in range(m):
    u, v = read_ints()
    adj[u - 1].append(v - 1)
    adj[v - 1].append(u - 1)

INF = int(1e9)
vis = [[False] * n for _ in range(1 << n)]
ans = [INF] * (1 << n)
ans[0] = 0
dq = deque()
for i in range(n):
    ans[1 << i] = 1
    vis[1 << i][i] = True
    dq.append((1 << i, i, 1))

while len(dq) > 0:
    state, u, dis = dq.popleft()
    for v in adj[u]:
        nxt = state ^ (1 << v)
        if not vis[nxt][v]:
            ans[nxt] = min(ans[nxt], dis + 1)
            vis[nxt][v] = True
            dq.append((nxt, v, dis + 1))

print(sum(ans))
