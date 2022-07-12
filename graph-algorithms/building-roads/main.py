n, m = map(int, input().split())
father = [i for i in range(n)]
size = [1 for i in range(n)]
root = {i for i in range(n)}
for i in range(m):
    a, b = map(int, input().split())
    _a, _b = a - 1, b - 1
    while _a != father[_a]:
        _a = father[_a]
    while _b != father[_b]:
        _b = father[_b]
    if _a != _b:
        if size[_a] < size[_b]:
            father[_a] = _b
            size[_b] += size[_a]
            root.remove(_a)
        else:
            father[_b] = _a
            size[_a] += size[_b]
            root.remove(_b)
print(len(root) - 1)
root = list(root)
for i in range(len(root) - 1):
    print(root[i] + 1, root[i + 1] + 1)