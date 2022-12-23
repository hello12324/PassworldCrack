struct NeuralNetwork {
    // 存储边的权值
    weights: Vec<Vec<f32>>,
    // 存储每个节点的偏移量
    biases: Vec<Vec<f32>>,
}

impl NeuralNetwork {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights = (0..hidden_size)
            .map(|_| (0..input_size).map(|_| rng.gen::<f32>()).collect())
            .collect();
        let biases = (0..output_size)
            .map(|_| (0..hidden_size).map(|_| rng.gen::<f32>()).collect())
            .collect();
        Self { weights, biases }
    }

    // 实现前馈算法
    fn forward(&self, inputs: &[f32]) -> Vec<f32> {
        let mut hidden = vec![0.0; self.weights.len()];
        // 计算隐藏层的输出
        for (i, weight) in self.weights.iter().enumerate() {
            hidden[i] = sigmoid(dot(weight, inputs) + self.biases[0][i]);
        }
        // 计算输出层的输出
        let output = dot(&self.weights[1], &hidden) + self.biases[1][0];
        vec![output]
    }

    // 实现反向传播算法
    fn backward(&mut self, inputs: &[f32], target: f32, learning_rate: f32) {
        let mut hidden = vec![0.0; self.weights.len()];
        // 计算隐藏层的输出
        for (i, weight) in self.weights.iter().enumerate() {
            hidden[i] = sigmoid(dot(weight, inputs) + self.biases[0][i]);
        }
        // 计算输出层的输出
        let output = dot(&self.weights[1], &hidden) + self.biases[1][0];
        // 计算输出层的误差
        let error = output - target;
        // 计算隐藏层的误差
        let hidden_error = elementwise_mul(
            &self.weights[1],
            &vec![error * sigmoid_prime(output)],
);
// 更新输出层的权值和偏移量
self.weights[1] = elementwise_sub(
&self.weights[1],
&vec![error * hidden[0] * learning_rate],
);
self.biases[1][0] -= error * learning_rate;
// 更新隐藏层的权值和偏移量
self.weights[0] = elementwise_sub(
&self.weights[0],
&vec![hidden_error[0] * inputs[0] * learning_rate],
);
self.biases[0][0] -= hidden_error[0] * learning_rate;
}
}

// 计算两个向量的内积
fn dot(v1: &[f32], v2: &[f32]) -> f32 {
v1.iter().zip(v2).map(|(x, y)| x * y).sum()
}

// 计算两个向量的元素级别的乘积
fn elementwise_mul(v1: &[f32], v2: &[f32]) -> Vec<f32> {
v1.iter().zip(v2).map(|(x, y)| x * y).collect()
}

// 计算两个向量的元素级别的差
fn elementwise_sub(v1: &[f32], v2: &[f32]) -> Vec<f32> {
v1.iter().zip(v2).map(|(x, y)| x - y).collect()
}

// Sigmoid 激活函数
fn sigmoid(x: f32) -> f32 {
1.0 / (1.0 + (-x).exp())
}

// Sigmoid 函数的导数
fn sigmoid_prime(x: f32) -> f32 {
let s = sigmoid(x);
s * (1.0 - s)
}
