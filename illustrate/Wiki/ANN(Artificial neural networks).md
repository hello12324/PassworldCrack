## ANN

ANN全称为Artificial neural networks,中文为人工神经网络,人工神经网络用于多种应用,例如机器视觉,图像处理,自动驾驶,聊天机器人(如chatGPT,使用了transformers语言模型)等领域,本质上是模拟了人脑中的神经元与神经元组成的神经网络的运行. 数学公式应该为 
加权的公式.

```math
\sum_{=1}^n w_i x_i
```

其中w_i是第i个输入的权值,x_i是第i个输入值,n是输入的数量,加权和公式用于计算神经元的输入信号.

激活函数公式(Sigmoid函数).

```math
f(x) = \frac{1}{1+e^{-x}}
```

> 激活函数不止一种还有很多这里只说一种.

误差函数

```math
E(y, \hat{y})
```

误差函数用于衡量神经网络的预测精准度.

梯度下降算法是

```math
w_i \gets w_i - \eta \frac{\partial E}{\partial w_i}
```

梯度下降算法是用于优化神经网络参数. 其中

```math
\eta
```

为学习率.

```math
\frac{\partial E}{\partial w_i}
```
表示损失函数关于w_i的偏导数.
> 稍后我会仔细的研究人工神经网络(或等到放假再研究).
