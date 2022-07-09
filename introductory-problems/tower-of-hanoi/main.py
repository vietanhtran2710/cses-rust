moves, count = [], 0

def move(n, start, end):
    global count
    s = set([1, 2, 3])
    s.remove(start)
    s.remove(end)
    if n == 1:
        count += 1
        moves.append((start, end))
    else:
        middle = s.pop()
        move(n - 1, start, middle)
        move(1, start, end)
        move(n - 1, middle, end)

n = int(input())
move(n, 1, 3)
print(count)
for item in moves:
    print(item[0], item[1])
