s = input()
cnt = 0
for c in s:
    cnt += int(c)
print('Yes' if cnt % 9 == 0 else 'No')
