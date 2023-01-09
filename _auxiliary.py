# This python file is only used for simple auxiliary generation of characters and other operations.
#                                                   make by geumbo

import base64
import hashlib
import os
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

    logo3 = """
     ______________
    |AntiTimming \x1b[41m:)\x1b[0m|
    |\x1b[32m:(\x1b[0m  Attack    |
     --------------
            \x1b[31mMerry\x1b[0m \x1b[32mChristmas!\x1b[0m
                make by geumbo
    """

    other = """
\x1b[33m(tips!)\x1b[0m This program has a parameter mode that can be used, just enter the parameters in the terminal!
    Usage:
        [Program file] [args]
        \x1b[33m(tips!)\x1b[0m Please enter the \x1b[32mhelp\x1b[0m command to get more instructions!
"""

    command = """
    Usage:
        --version                 Do you want to see the version? My program does not have the concept of version, if you want to see it, please type this command.  
        --help  h                 If you fucking want more help type this get command arguments.
        --delay  d                Too fast? Want something a little safer? Then trade some performance for safety? Set the delay.
        --length len              What? Do you think the password is too long? Then set this!
        --FilePath(FP) file_path  Do you think it is slow? Then create a file to set commonly used characters!
        --passworld pass          the passworld file(default file is current path 'passworld.txt')
    """
    version = """
    \x1b[33m(Version)\x1b[0m
                No fucking Version\x1b[41m:)\x1b[0m.
    """
    command1 = """
    Usage:
        [the program] [the first strings] [the second strings]
    """
    logoL = []
    logoL.append(logo)
    logoL.append(logo1)
    logoL.append(logo2)
    logoL.append(logo3)
    logoL.append(other)
    logoL.append(command)
    logoL.append(command1)
    logoL.append(version)

    for i in logoL:
        [print(i0)for i0 in str(base64.encodebytes(bytes(i.encode()))).split("\\n")]

def TimingAttack():

    with open(os.path.dirname(__file__)+"/TIMING.txt", "r") as f:
        a = f.readlines()
        num = 0;
        lines_ = 0;
        for i in a:
            num +=int(i);
            lines_ +=1;

        print(num/lines_);

if __name__ == "__main__":
    #hash_gen(bytes(input().encode()));
    base_transformation()
    pass;
