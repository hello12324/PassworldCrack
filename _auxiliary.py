#这个python文件只是用于简单的辅助生成字符等操作.
# This python file is only used for simple auxiliary generation of characters and other operations.
#                                                   make by geumbo

import base64
import hashlib
def hash_gen(hash_):
    a = hashlib.sha256(hash_)
    print(a.hexdigest());
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
        --version                 Do you want to see the version? My program does not have the concept of version, if you want to see it, please type this command.  
        --help  help              If you fucking want more help type this get command arguments.
        --dely  d                 Too fast? Want something a little safer? Then trade some performance for safety? Set the delay.
        --length len              What? Do you think the password is too long? Then set this!
        --FilePath(FP) file_path  Do you think it is slow? Then create a file to set commonly used characters!
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
    hash_gen(b"pa;");
    pass;