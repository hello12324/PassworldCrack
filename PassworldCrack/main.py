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
    other = """
\x1b[33m(tips!)\x1b[0m This program has a parameter mode that can be used, just enter the parameters in the terminal!
    Usage:
        [Program file] [args]
        \x1b[33m(tips!)\x1b[0m Please enter the help command to get more instructions!

    """
    command = """
    Usage:
        --version                 你想看版本吗? 我的程序并没有版本这个概念,如果你想看的话请键入这个指令.  
        --help  help              如果你他妈的想要获取更多帮助就输入这个获取指令参数
        --dely  d                 太快了?想要更安全一些吗？那就减一些性能换取安全吧?设置延迟
        --length len              什么？你觉得密码太长吗? 那就设置这个吧!
        --FilePath(FP) file_path  你觉得慢吗?那就创建个文件设置常用的字符吧!
    """
    logoL = []
    logoL.append(logo)
    logoL.append(logo1)
    logoL.append(logo2)
    logoL.append(other)
    for i in logoL:
        encode = base64.encodebytes(bytes(command.encode()))
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
    base_transformation();
    pass;