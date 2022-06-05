from sys import stdin
def input(): return stdin.readline().strip()


def read_int():
    return int(input())


def read_ints():
    return map(int, input().split())


class Trie:
    def __init__(self):
        self.children = [None, None]
        self.ca = 0
        self.cb = 0


def dfs(u, p):
    if not u:
        return 0

    lc = dfs(u.children[0], u)
    rc = dfs(u.children[1], u)
    cost = lc + rc

    eliminated = min(u.ca, u.cb)
    u.ca -= eliminated
    u.cb -= eliminated
    if u.ca > 0:
        p.ca += u.ca
        cost += u.ca
    elif u.cb > 0:
        if u == p.children[0]:
            p.cb += u.cb
            cost += u.cb
        else:
            print(-1)
            exit()

    return cost


n = read_int()
root = Trie()
root.children[1] = Trie()
for ai in read_ints():
    if ai == 0:
        root.ca += 1
    else:
        cur = root
        for bit in map(int, bin(ai)[2:]):
            if not cur.children[bit]:
                cur.children[bit] = Trie()
            cur = cur.children[bit]
        cur.ca += 1

for bi in read_ints():
    if bi == 0:
        root.cb += 1
    else:
        cur = root
        for bit in map(int, bin(bi)[2:]):
            if not cur.children[bit]:
                cur.children[bit] = Trie()
            cur = cur.children[bit]
        cur.cb += 1

print(dfs(root, None))
