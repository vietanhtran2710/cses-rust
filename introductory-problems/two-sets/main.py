from math import ceil, sqrt
n = int(input())
if n * (n + 1) % 4 != 0:
    print("NO")
else:
    print("YES")
    c = -n * (n + 1) // 2
    delta = 1 - 4 * c
    x1 = ceil((-1 + sqrt(delta)) / 2)
    first_sum = x1 * (x1 + 1) // 2
    second_sum = n * (n + 1) // 2 - first_sum
    diff = abs(first_sum - second_sum) // 2
    if diff == 0:
        print(x1)
        print(" ".join(map(str, range(1, x1 + 1))))
        print(n - x1)
        print(" ".join(map(str, range(x1 + 1, n + 1))))
    else:
        print(x1 - 1)
        print(" ".join(map(str, list(range(1, diff)) + list(range(diff + 1, x1 + 1)))))
        print(n - x1 + 1)
        print(" ".join(map(str, [diff] + list(range(x1 + 1, n + 1)))))