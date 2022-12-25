import base64

def base_transformation():
    logo = """
        ____________
    | passw0rld\x1b[31m:)\x1b[0m|
    |  \x1b[32m:(\x1b[0mPr0gram |
        ------------
            \x1b[31mMerry\x1b[0m \x1b[32mChristmas!\x1b[0m
                make by geumbo
    """
    logo1 = """
        _____________________
    |Leibniz \x1b[32m:(\x1b[0m           |
    |    \x1b[31m:)\x1b[0m transformation|
        ---------------------
            \x1b[31mMerry\x1b[0m \x1b[32mChristmas!\x1b[0m\n
                    make by geumbo
    """
    logo2 = """
        ____________
    |Passw0rld \x1b[41m:)\x1b[0m|
    |  \x1b[32m:(\x1b[0m  Crack |
        ------------
            \x1b[31mMerry\x1b[0m \x1b[32mChristmas!\x1b[0m
                make by geumbo
    """
    logoL = []
    logoL.append(logo)
    logoL.append(logo1)
    logoL.append(logo2)
    for i in logoL:
        encode = base64.encodebytes(bytes(i.encode()))
        print(encode)

def genACSII(x, y):
    for i in range(x):
        for j in range(y):
            if i == 0 or j==0 or i==x-1 or j==y-1:
                print("{}", end="")
            else:
                print("{}", end="")
        print()
if __name__ == "__main__":
    pass;