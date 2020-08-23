def read_ints():
    return map(int, input().split(' '))


n, x, t = read_ints()
print(((n - 1) // x + 1) * t)
