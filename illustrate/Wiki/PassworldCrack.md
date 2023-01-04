# PassworldCrack

这是使用rust语言编写的破解密码程序,不需要庞大的密码字典就可以破解密码(使用重复顺序排列组合)只需要一个字符串文件(可自定义字符串,默认为93个字符串),但是效率有一定的问题,我会重写优化. 这里说的重复顺序排列组合是这样,假如有3个字符破解一个2位数密码就是这样 aa ab ac ba bb bc ca cb cc 长度就是9 数学公式为

```math
f(stringLen, PassworldLen) = stringLen^PassworlLen
```

> 这就是对PassworldCrack进行简单的解释. 程序需要几个文件,程序本体,'DefaultPassworld.txt '(定义默认字符串), 'passworld.txt'(hash密码文件),文件需要和程序本体在同一个目录下,或者在终端所在的目录下(你可能不明白,你可以自己琢磨研究以下).

