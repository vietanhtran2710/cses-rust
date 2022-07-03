count = 0
reserved, current = set(), []
for i in range(8):
    line = input()
    for index, char in enumerate(line):
        if char == "*":
            reserved.add((i, index))

def go(index):
    global count
    if index == 8:
        count += 1
    else:
        for i in range(8):
            if (index, i) not in reserved:
                valid = True
                for j in range(index):
                    if i == current[j] or abs(i - current[j]) == abs(index - j):
                        valid = False
                        break
                if valid:
                    current.append(i)
                    go(index + 1)
                    current.pop()

go(0)

print(count)