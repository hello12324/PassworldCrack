# 解释
 > 这是我练习的项目,用于学习rust语言的,虽然写的不怎么好,但是我会努力的!.
 > 如果你觉得我的代码有问题请
 	 My email
		testsendkfotesycike@gmail.com
   到我的邮箱交流!

 > PassworldCarck 实际上我是想把这个功能实现在arduino编程板上面(利用HID漏洞(Human Interface Device)),因为编程板上面不可以填入超级多的字节所以我想实现自动实时生成密码的项目,现在已经实现了,使用rust语言,稍后我会利用在arduino上面实现,并公开代码到这个仓库.

# 我的项目
 ## ANN(Artificial Neural Network)
 这里是我简单写的人工神经网络,现在只是雏形,目前不可执行.
 ## AntiTimmingAttack
 这是一个反计时攻击的项目,计时攻击是一种密码攻击,通过计算密码程序的返回时间来估计密码的正确性.详细请看我的MyIdea仓库.
 ## leibniz_transformation
 莱布尼茨变换是一种数学方法, 用于将一个时间函数转换为其复平面函数的表示. 莱布尼茨变换最常用来解决常微分方程(differential equations),也常用来分析电气和电子系统中的信号和系统动态行为.
 具体来说, 莱布尼茨变换可以将时间函数中的微分项和积分项转换为柿子(complex functions)的乘积和和, 这样就可以用线性代数的方法来解决问题了. 莱布尼茨变换也可以用来分析系统的稳态行为(steady-state behavior), 因为在稳态情况下, 系统的状态不再变化, 可以使用复数谱分析(complex spectra analysis)来研究系统的性质.,不一定正确,但是大概就是这样了.
 ## PassworldCrack
 	这是一个不需要字典实时生成密码的项目,只需要一个"DefaultPassworld.txt"文件(文件需要和程序在一个目录或者在终端的路径下,但是运行时可以自定义路径),文件内部有94个字符(也可以自定义常用字符)就可以生成密码,利用到了离散数学中的基本应用,组合数学(Combinatorics),如下演示

	- 假如一共有3个字符,要破解一个2位数密码,排列组合如下
			 --
		1. aa      |
		2. ab      |
		3. ac      |
		4. ba      |
		5. bb      |   =>[一共就有3的2次方]
		6. bc      |
		7. ca 	   | 	
		8. cb      |
		9. cc      |
                         --
	利用到了直线排列就是考虑顺序的,所以效率还是有问题.
 ## PassworldProgram
 这是一个带密码的程序,只是简单的编写测试,例如时攻击测试等等.
 ## C Language Projects
 ### PassworldCrack Rewritten version
 这是rust语言PassworldCrack项目的重写版本,因为底层语言的效率比rust语言更高校(可能rust语言代码部分优化问题),总之我准备用C语言重写这个项目,功能性可能和原来的差不多.
 现在主要功能实现了,不过有问题
 ### PassworldCrack ArduinoLeomardo Version
 这是rust语言PassworldCrack项目的Arduino版本,让暴力破解用于生活中,利用HID(Human Interface Divce)漏洞.

# 使用方法
 ## PassworldCrack
 本项目的执行程序需要的文件如下,你在下载本执行程序应该可以看到这些文件或者在release.
 - ```DefaultPassworld.txt```  密码破解需要的基本字符串,默认有所有的字符串,可以修改为常用的字符串.
 - ```passworld.txt```   保存hash256位密码用于破解.
 文件需要可执行文件在同一个目录或者在终端指令的目录下(你可以不会明白).
 如果你要修改本项目代码并执行的话请保证本地包含cargo和[rustup](https://rustup.rs/). 如果你已经安装只需要执行 ```cargo run``` 指令就可以编译运行. 如果你只是想编译请执行 ```cargo build --release``` 指令, 执行文件会保存在 ```./target/release``` 目录下.
 ## PassworldProgram
 本项目需要的文件.
 - ```passworld.txt``` 保存256位密码用于保存密码.
 这是一个简单的密码程序,仅仅用于测试,只是玩玩. 默认的密码为'passworld'. 如果你要生成其他的hash密码使用的话, 你可以执行我的python脚本内的hash_gen函数. 文件名为'_auxiliary.py',请自行修改执行.

# 协助贡献代码方法
 可以到我的邮箱上和我进行交流,同意后即可贡献代码.

# 使用的协议.
 使用了麻省理工协议.

> 这里面的项目想看详细内容请看release!.
> 这里面的项目没有任何利用价值,只是玩玩.





> 还有一句话! Merry Cristmas!




> 今年马上就要过了,我们终究都要老去,但是我们的意志永远存在! 加油!
