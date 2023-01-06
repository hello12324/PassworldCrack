//
//     ___        __  _ _______             _           ___  __  __           __  
//    / _ | ___  / /_(_)_  __(_)_ _  __ _  (_)__  ___ _/ _ |/ /_/ /____ _____/ /__
//   / __ |/ _ \/ __/ / / / / /  ' \/  ' \/ / _ \/ _ `/ __ / __/ __/ _ `/ __/  '_/
//  /_/ |_/_//_/\__/_/ /_/ /_/_/_/_/_/_/_/_/_//_/\_, /_/ |_\__/\__/\_,_/\__/_/\_\ 
//                                          /___/                             
//
// _____________________________________________________________________________________________
//| Timing Attack is a password attack method that judges the correctness of the password       | 
//| by calculating and counting the past password verification response time, and this code is  | 
//| to prevent the problem that the password verification response time is not the same,        |
//| essentially adding a judgment to make the response of the same time.                        |
//|                                                                                             |
//| 2022-12-26 <My emal>testsendkfotesycike@gmail.com                                           |
//| 	Merry Christmas!                                                                        |
//|                                                              <Make by GeumBo>               |
// ---------------------------------------------------------------------------------------------
use std::{
    time::{
        Instant,
        Duration
    },
    io::Write,
    fs::*,
    env
};

//异或运算,编程语言中符号为'^',运算基本原理是这样 => False(0) ^ False(0) = False(0), True(1) ^ True(1) = True(1), False(0) ^ True(1) = True(1), True(1) ^ False(0) = True(1)
//XOR operation, the symbol in programming language is '^', the basic principle of operation is like this => False(0) ^ False(0) = False(0), True(1) ^ True(1) = True(1 ), False(0) ^ True(1) = True(1), True(1) ^ False(0) = True(1)
fn xor(bytes1: String, bytes2: String) -> i64 {
    //定义动态向量数组.
    //Define Vector arry.
    let mut a = Vec::new();
    let mut b = Vec::new();

    //将字符转ascii码.
    // Convert the character to ascii code.
    for i0 in bytes1.as_bytes(){
        let c0 = *i0 as u8;
        a.push(c0);
    }
    for i1 in bytes2.as_bytes(){
        let c1 = *i1 as u8;
        b.push(c1);
    }

    //将字符类型转为int类型.
    //Convert character type to int type.
    let i64_0: Result<i64, _> = a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("").trim().parse();
    let i64_1: Result<i64, _> = b.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("").trim().parse();
    let _i64 = i64_0.unwrap_or(0);
    let _gi64 = i64_1.unwrap_or(0);

    //返回result.
    //return result.
    return _i64 ^ _gi64;
}

//密码匹配函数
//说明
//首先会将两个字符串(a和b变量)的长度进行比较,如果长度不相同则直接返回false(0),如果相同则进行异或比较让其返回的时间相同或者完全不一样,类似这样
// 97 ^ 97 = 0
// 但是注意这里等于0(false)所以需要和一个值0和这个结果匹配结果就是
// 异或结果     定义值   
//    0           0
// 这样就相等就会返回true(1)

// password matching function
//illustrate
//First, the lengths of the two strings (a and b variables) will be compared, and if the lengths are not the same, false (0) will be returned directly, and if they are the same, an XOR comparison will be performed to make the returned time the same or completely different, like this
// 97^97 = 0
// But note that this is equal to 0 (false), so it needs to be matched with a value of 0 and this result is
// XOR result       definition value
// 0                        0
// This equals and returns true(1).
fn _SAFE_EQU(a: String, b: String) -> bool{
    if a.len() != b.len() {
        return false;
    }
    let mut pass = 0;
    let mut status = 0;
    loop{
        if 0 < a.len() {
            pass = xor(a.to_string(), b.to_string());
            break;
        }
    }
    return pass == 0;

}



//写入操作,非主要函数,仅仅是用作记录每次的时间差.
//Write operation, non-main function, just used to record the time difference each time.
fn _file(contents: String,FNP: String)
{
    let current_dir = env::current_dir().unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(current_dir.join(FNP)).expect("[\x1b[41m:(\x1b[0m] Failed");
    file.write_all(contents.as_bytes()).expect("[\x1b[41m:(\x1b[0m] Failed");

}

//测试
//test
fn main(){ 
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
    //_file(asc.to_string(), "TIMING.txt".to_string()); 
    }else{
        let startTime = Instant::now();
        let func_ = _SAFE_EQU("PASSWORLD".to_string(), args[1].to_string());
        let overTime = Instant::now();
        let time = overTime - startTime;
        println!("{:?}, Time:{}",func_,time.as_nanos());
        let asc = format!("{}\n", time.as_nanos());
    }
    
}