n = int(input())
bits = ["0", "1"]
for i in range(2, n + 1):
    reserve = list(map(lambda x: "1" + x, bits[::-1]))
    bits = list(map(lambda x: "0" + x, bits))
    bits += reserve
for item in bits:
    print(item)