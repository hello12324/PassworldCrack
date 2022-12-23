def draw_square(x, y):
    for i in range(x):
        for j in range(y):
            if i == 0 or j==0 or i==x-1 or j==y-1:
                print("{}", end="")
            else:
                print("{}", end="")
        print()

draw_square(10, 10)

