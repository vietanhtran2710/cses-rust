n = int(input())
arr = list(map(int, input().split()))
s = sum(arr)
m = s + 1

def go(index, _s):
    global m
    diff = abs(s - _s - _s) 
    if diff < m:
        m = diff
    for i in range(index + 1, len(arr)):
        go(i, _s + arr[i])

go(-1, 0)
print(m)