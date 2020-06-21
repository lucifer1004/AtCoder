from math import pi, sin, cos


def read_int():
    return int(input())


deg = read_int()
x = 0.0
y = 0.0
theta = 90.0
rad = 180.0 / pi
cnt = 0
while True:
    cnt += 1
    x += cos(theta / rad)
    y += sin(theta / rad)
    theta += deg
    if x * x + y * y <= 1e-6:
        break

print(cnt)
