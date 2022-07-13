n = int(input())
for i in range(n):
    Ax, Ay, Bx, By, X, Y = map(int, input().split())
    value = (Bx - Ax) * (Y - Ay) - (By - Ay) * (X - Ax)
    print("LEFT" if value > 0 else "RIGHT" if value < 0 else "TOUCH")