from itertools import permutations
n = "".join(sorted(input()))
output = set(permutations(n))
print(len(output))
for item in sorted(list(output)):
    print("".join(item))
