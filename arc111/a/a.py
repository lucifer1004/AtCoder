n, m = map(int, input().split())
p = pow(10, n, m)
q = pow(10, n, m * m)
print((q - p) // m)
