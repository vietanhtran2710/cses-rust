from collections import Counter
n, result, odd_count, odd = input(), "", 0, ""
dct = Counter(n)
for key, value in dct.items():
    if value % 2 != 0:
        if odd_count == 1:
            print("NO SOLUTION")
            break
        else:
            odd_count += 1
            odd = key * value
    else:
        result += key * (value // 2)
print(result + odd + result[::-1])
