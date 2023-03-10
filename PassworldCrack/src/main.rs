//
//     ____                           ____       __    __    ______                __  
//    / __ \____ ____________      __/ __ \_____/ /___/ /   / ____/________ ______/ /__
//   / /_/ / __ `/ ___/ ___/ | /| / / / / / ___/ / __  /   / /   / ___/ __ `/ ___/ //_/
//  / ____/ /_/ (__  |__  )| |/ |/ / /_/ / /  / / /_/ /   / /___/ /  / /_/ / /__/ ,<   
// /_/    \__,_/____/____/ |__/|__/\____/_/  /_/\__,_/____\____/_/   \__,_/\___/_/|_|  
//                                                 /_____/                            
//
// ______________________________________________________________________________ 
//| Use the simplest combinatorial algorithm in the second grade                 |
//|   of middle school (is it an algorithm? Haha) to achieve real-time generation| 
//|      of characters with only 93 or less characters without a dictionary.     |
//|                                                                              |
//| 2022-12-26 <My emal>testsendkfotesycike@gmail.com                            |
//|      Marry Christmas!                                                        |
//|                                                      <Make by GeumBo>        |
//|                                                                              |
// ------------------------------------------------------------------------------
//


/*
illustrate
     This program does not have any use value, it is just written for fun.
     The function of this program is not to use the generated dictionary to use passwords, but to automatically report characters to generate passwords in real time, although this idea is still problematic.
            make by geumbo
*/
//调用的模块
//Called module
use rand::{thread_rng, Rng}; //随机模块,需要安装 random module, Need to install.
use std::{  //基本模块 default module
    time::Duration, //时间函数，用于计时.  
    thread, //进程模块,用于关闭程序,多进程等. Thread module
    fs::*, //文件操作模块. file operation module
    io,//I/O读取操作模块,用于输入输出等操作. input/output module
    io::Write, //I/O 文件写入函数调用. file io write module
    //env, //env模块,用于接受环境的数据参数等. accept environmental data
    env::*, //接受命令行参数. Accept command parameters.
    io::ErrorKind, //错误,如果执行程序报错时使用它接收错误. catch error.
    sync::{
        mpsc,
        Arc,
        Mutex
    },
};
//hash加密和md5加密. hash encoding and md5 encoding.
use crypto::{
    sha2::Sha256,
    digest::Digest,
    md5::Md5 
};
use base64::{ //base64加密,只是用于将base code of logo解析 base64 encoding, just only decoding logo strings.
    encode,
    decode
};
//第三方时间模块,用于记录密码成功的时间. A third-party module, used to save the successful password record time
use chrono::{DateTime, Local, TimeZone};

use futures::executor::block_on;
//内部的代码可能稍微有点不是很人性化:).
//The internal code may be a bit impersonal :).


// hash加密函数  the hash encoding function.
fn hash_encryt_256(hash_value: String) -> String
{
    let mut hasher = Sha256::new();
    let _content = String::from(hash_value);
    hasher.input_str(&_content);
    return hasher.result_str()
}
// the md5 encoding function.
fn md5_encryt(md5_value: String) -> String
{
    let mut md_5 = Md5::new();
    let _content = String::from(md5_value);
    md_5.input_str(&_content);
    return md_5.result_str()
}

//输出的规格,用于测试密码的输出和规格. 当然函数如果密码输入成功的话就会保存.
//The specification of the output, used to test the output and specification of the password. Of course, the function will be saved if the password is entered successfully.
fn output_style(_passworld: String, _time_sleep: Duration, PASSWORLD: String)
{
    let now: DateTime<Local> = Local::now();
    let formatted = now.format("[%Y-%m-%d %I:%M%P]").to_string();   
    let _1passworld = _passworld.clone();
    let mut rng = thread_rng();
    let a = rng.gen_range(31, 37);
    print!("[\x1b[32m:)\x1b[0m \x1b[34mTEST OUTPUT\x1b[0m] [\x1b[{fuck}m{}\x1b[0m] -> [\x1b[{fuck}m{hash_1}\x1b[0m]\r\r",_passworld,fuck=a,hash_1 = hash_encryt_256(_passworld.clone()));thread::sleep(_time_sleep);
    let passworld_ = passworldTEST(PASSWORLD, _passworld,"sha256".to_string());
    if passworld_ == true{
        println!("[\x1b[32m:)\x1b[0m]Passworld Success, Passworld is \x1b[34m{}\x1b[0m and value is \x1b[34m{hash_2}\x1b[0m", &_1passworld, hash_2 = hash_encryt_256(_1passworld.clone()));
        let save = format!("
                [{}]
        [PASSWORLD]  |  [VALUE]
            {}               {}
        
        ", formatted, _1passworld, hash_encryt_256(_1passworld.clone()));
        _file(save.to_string(), "PASSWORLD_SUCCESS.txt".to_string());
        std::process::exit(0);
    }
}

//密码匹配机制,如果你觉得有点小题大作了的话可以完全修改,实际上不需要pass变量保存成功失败变量去匹配,可以直接使用if语句去匹配两个值的相同性.
//Password matching mechanism, if you think it’s a bit of a fuss, you can completely modify it. In fact, you don’t need the pass variable to save the success or failure variable to match. You can directly use the if statement to match the identity of the two values.
fn passworldTEST(pasphrase: String, Enum: String, passworld_style: String) -> bool
{ 
    if (passworld_style) == "sha256".to_string() {
        if hash_encryt_256(Enum) == (pasphrase) {
            return true;
        }else{
            return false;
        }
    }else if (passworld_style) == "md5".to_string() {
        if md5_encryt(Enum) == (pasphrase) {
            return true
        }else{
            return false
        }
    }else {
        return false
    }
} 
//文件写入,如果文件已存在就追加写入.
//Write to the file, append if the file already exists.
fn _file(contents: String,FNP: String)
{
    let current_dir = current_dir().unwrap();
    //write(current_dir.join(FNP), contents);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(current_dir.join(FNP)).expect("[\x1b[41m:(\x1b[0m] Failed");

    // 写入内容 write content.
    file.write_all(contents.as_bytes()).expect("[\x1b[41m:(\x1b[0m] Failed");

}

//读取文件,如果用户自定义了自己的字符文件就可以使用这个函数去调用.
//Read the file, if the user has customized his own character file, he can use this function to call.
fn read_fileF(Default_: String,FP: String) -> Result<String, std::io::Error> 
{
    let current_dir_1 = current_dir().unwrap();
    let current_file = current_dir_1.join(Default_);
    let content = read_to_string(FP);
    match content
    {
        Ok(t) => {
            Ok(t)
        }
        Err(f) => {
            if f.kind() == ErrorKind::NotFound {
                read_to_string(current_file)
            }
            else{
                Err(f)
            }
        }
    }
}

//主要函数 main function.
fn _func(time_Sleep: u64, FNP: String, count_1: i32, _PASSWORLD_1: String)
{
    let _PASSWORLD = &_PASSWORLD_1.trim();
    let time_sleep = Duration::from_millis(time_Sleep);
    //let strings = format!("{}{}","abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\\{\\}[]:;'<,>.?/",'"');
    let strings = read_fileF("DefaultPassworld.txt".to_string(),FNP.to_string()).expect("[\x1b[41m:(\x1b[0m] '\x1b[34mDefaultPassworld.txt\x1b[0m' file is NotFound, please create this file on program dir path and add content 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\\{//}[]:;'<,>.?/'{}' or add your want add custom content[没有找到'\x1b[34mDefaultPassworld.txt\x1b[0m',请手动创建文件到程序的目录下并添加内容'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\\{//}[]:;'<,>.?/'{}'或者添加自定义内容] [\x1b[42m:)\x1b[0m]  ");
    println!("[\x1b[32m:)\x1b[0m] file content is {}",strings);
    let mut length_ = 0;
    for length in strings.chars() 
    {
        length_ += 1;
    }
    println!("[\x1b[32m:)\x1b[0m] string length is \x1b[32m {}\x1b[0m bytes",length_);
    thread::sleep(Duration::from_millis(1000));
    //println!("{}", strings);
    let mut num = 0;
    for i0 in 1..=count_1
    {
        if i0 == 1
        {
            for c0 in strings.chars()
            {
                num +=1;
                let passworld = c0.to_string();
                output_style(passworld, time_sleep, _PASSWORLD.to_string());
            }
        }else if i0 == 2
        {
            for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    num +=1;
                    let passworld = c0.to_string()+&c1.to_string();
                    output_style(passworld, time_sleep, _PASSWORLD.to_string());
                }
            }
        }else if i0 == 3
        {
                for c0 in strings.chars()
                {
                    for c1 in strings.chars()
                    {
                        for c2 in strings.chars()
                        {
                            let passworld = c0.to_string()+&c1.to_string()+&c2.to_string();
                            num +=1;
                            output_style(passworld, time_sleep, _PASSWORLD.to_string());
                        }
                    }
                }
        } else if i0 == 4
        {
                for c0 in strings.chars()
                {
                    for c1 in strings.chars()
                    {
                        for c2 in strings.chars()
                        {
                            for c3 in strings.chars()
                            {
                                let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string();
                                num+=1;
                                output_style(passworld, time_sleep, _PASSWORLD.to_string());
                            }
                        }
                    }
                }
        } else if i0 == 5 {
            for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string();
                                num+=1;
                                output_style(passworld, time_sleep, _PASSWORLD.to_string());
                            }
                        }
                    }
                }
            }
        } else if i0 == 6 {
            for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string();
                                    num+=1;
                                    output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                }
                            }
                        }
                    }
                }
            }
        } else if i0 == 7 {
            for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        
                                    let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string();
                                    num+=1;
                                    output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if i0 == 8 {
            for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string();
                                            num+=1;
                                            output_style(passworld, time_sleep, _PASSWORLD.to_string());

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 9 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string();
                                                num+=1;
                                                output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 10 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string();
                                                    num+=1;
                                                    output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 11 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars(){
                                                        let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string();
                                                        num+=1;
                                                        output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 12 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars(){
                                                            let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string();
                                                            num+=1;
                                                            output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 13 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars()
                                                        {
                                                            for c12 in strings.chars()
                                                            {
                                                                let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string()+&c12.to_string();
                                                                num+=1;
                                                                output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 14 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars()
                                                        {
                                                            for c12 in strings.chars()
                                                            {
                                                                for c13 in strings.chars()
                                                                {
                                                                    let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string()+&c12.to_string()+&c13.to_string();
                                                                    num+=1;
                                                                    output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 15 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars()
                                                        {
                                                            for c12 in strings.chars()
                                                            {
                                                                for c13 in strings.chars()
                                                                {
                                                                    for c14 in strings.chars()
                                                                    {
                                                                        let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string()+&c12.to_string()+&c13.to_string()+&c14.to_string();
                                                                        num+=1;
                                                                        output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 16 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars()
                                                        {
                                                            for c12 in strings.chars()
                                                            {
                                                                for c13 in strings.chars()
                                                                {
                                                                    for c14 in strings.chars()
                                                                    {
                                                                        for c15 in strings.chars()
                                                                        {
                                                                            let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string()+&c12.to_string()+&c13.to_string()+&c14.to_string()+&c15.to_string();
                                                                            num+=1;
                                                                            output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                                        } 
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 17 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars()
                                                        {
                                                            for c12 in strings.chars()
                                                            {
                                                                for c13 in strings.chars()
                                                                {
                                                                    for c14 in strings.chars()
                                                                    {
                                                                        for c15 in strings.chars()
                                                                        {
                                                                            for c16 in strings.chars()
                                                                            {
                                                                                let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string()+&c12.to_string()+&c13.to_string()+&c14.to_string()+&c15.to_string()+&c16.to_string();
                                                                                num+=1;
                                                                                output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 18 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars()
                                                        {
                                                            for c12 in strings.chars()
                                                            {
                                                                for c13 in strings.chars()
                                                                {
                                                                    for c14 in strings.chars()
                                                                    {
                                                                        for c15 in strings.chars()
                                                                        {
                                                                            for c16 in strings.chars()
                                                                            {
                                                                                for c17 in strings.chars()
                                                                                {
                                                                                    let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string()+&c12.to_string()+&c13.to_string()+&c14.to_string()+&c15.to_string()+&c16.to_string()+&c17.to_string();
                                                                                    num+=1;
                                                                                    output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 19 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars()
                                                        {
                                                            for c12 in strings.chars()
                                                            {
                                                                for c13 in strings.chars()
                                                                {
                                                                    for c14 in strings.chars()
                                                                    {
                                                                        for c15 in strings.chars()
                                                                        {
                                                                            for c16 in strings.chars()
                                                                            {
                                                                                for c17 in strings.chars()
                                                                                {
                                                                                    for c18 in strings.chars()
                                                                                    {
                                                                                        let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string()+&c12.to_string()+&c13.to_string()+&c14.to_string()+&c15.to_string()+&c16.to_string()+&c17.to_string()+&c18.to_string();
                                                                                        num+=1;
                                                                                        output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                                                    }
                                                                                }
                                                                            }
                                                                        } 
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }else if i0 == 20 {
        for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    for c2 in strings.chars()
                    {
                        for c3 in strings.chars()
                        {
                            for c4 in strings.chars()
                            {
                                for c5 in strings.chars()
                                {
                                    for c6 in strings.chars()
                                    {
                                        for c7 in strings.chars()
                                        {
                                            for c8 in strings.chars()
                                            {
                                                for c9 in strings.chars()
                                                {
                                                    for c10 in strings.chars()
                                                    {
                                                        for c11 in strings.chars()
                                                        {
                                                            for c12 in strings.chars()
                                                            {
                                                                for c13 in strings.chars()
                                                                {
                                                                    for c14 in strings.chars()
                                                                    {
                                                                        for c15 in strings.chars()
                                                                        {
                                                                            for c16 in strings.chars()
                                                                            {
                                                                                for c17 in strings.chars()
                                                                                {
                                                                                    for c18 in strings.chars()
                                                                                    {
                                                                                        for c19 in strings.chars()
                                                                                        {
                                                                                            let passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string()+&c8.to_string()+&c9.to_string()+&c10.to_string()+&c11.to_string()+&c12.to_string()+&c13.to_string()+&c14.to_string()+&c15.to_string()+&c16.to_string()+&c17.to_string()+&c18.to_string()+&c19.to_string();
                                                                                            num+=1;
                                                                                            output_style(passworld, time_sleep, _PASSWORLD.to_string());
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                        }                                       
                                    }
                                }
                            }
                        }
                    }
                }
            }
    }
    else{
        println!("count \x1b[32m {}\x1b[0m", num);
        break;
    }
}
}

//程序执行函数Main
//the main excute function.
fn main() {
    println!("{}", String::from_utf8(decode("CiAgICBfX19fX19fX19fX18KICAgfFBhc3N3MHJsZCAbWzQxbTopG1swbXwKICAgfCAgG1szMm06KBtbMG0gIENyYWNrIHwKICAgIC0tLS0tLS0tLS0tLQogICAgICAgIBtbMzFtTWVycnkbWzBtIBtbMzJtQ2hyaXN0bWFzIRtbMG0KICAgICAgICAgICAgbWFrZSBieSBnZXVtYm8K").unwrap()).unwrap());
        let args: Vec<String> = args().collect();
        let mut Args_FP = "".to_string();
        let mut Args_Delay = 0;
        let mut Args_Length = 0;
        if args.len() == 1 {
            println!("{}", String::from_utf8(decode("ChtbMzNtKHRpcHMhKRtbMG0gVGhpcyBwcm9ncmFtIGhhcyBhIHBhcmFtZXRlciBtb2RlIHRoYXQgY2FuIGJlIHVzZWQsIGp1c3QgZW50ZXIgdGhlIHBhcmFtZXRlcnMgaW4gdGhlIHRlcm1pbmFsIQogICAgVXNhZ2U6CiAgICAgICAgW1Byb2dyYW0gZmlsZV0gW2FyZ3NdCiAgICAgICAgG1szM20odGlwcyEpG1swbSBQbGVhc2UgZW50ZXIgdGhlIGhlbHAgY29tbWFuZCB0byBnZXQgbW9yZSBpbnN0cnVjdGlvbnMhCgogICAg").unwrap()).unwrap());
            println!("[\x1b[32m:)\x1b[0m] Are You Sure continue?(Yes or No Default:Y)[是否继续 默认继续]");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("[\x1b[41m:(\x1b[0m]failed");
            input.truncate(input.len()-1);
            if input.trim() == "" || input.trim() == "Y"
            {
            println!("[\x1b[32m:)\x1b[0m] Passworld length(Default is 20)[密码的长度 默认20]");
            let mut count_2 = String::new();
            io::stdin().read_line(&mut count_2).expect("[\x1b[41m:(\x1b[0m]Failed");
            count_2.truncate(count_2.len()-1);
            let num_2: Result<i64, _> = count_2.trim().parse();
            let num_2_p = num_2.unwrap_or(-1);
            println!("[\x1b[32m:)\x1b[0m] Delay(default 20ms)[延迟, 默认为20ms]");
            let mut time_ = String::new();
            io::stdin().read_line(&mut time_).expect("[\x1b[41m:(\x1b[0m]Failed");
            time_.truncate(time_.len()-1);
            let num: Result<i64, _> = time_.trim().parse();
            let num_ = num.unwrap_or(-1);
            println!("[\x1b[32m:)\x1b[0m] Custom character file location (default./DefaultPassworld.txt), if you don't want to change, please skip.[自定义字符文件位置(默认./DefaultPassworld.txt),如果你不想更改请越过.]");
            let mut FNP_C = String::new();
            io::stdin().read_line(&mut FNP_C).expect("[\x1b[41m:(\x1b[0m]FAILED");
            FNP_C.truncate(FNP_C.len()-1);
            println!("[\x1b[32m:)\x1b[0m] Please Input HashPassworld(sha256)or MD5 File Path, Default file name is a 'passworld.txt'[请输入hash密码文件(sha256类型)或者MD5文件路径,默认'passworld.txt']");
            let mut PA_ = String::new();
            io::stdin().read_line(&mut PA_).expect("[\x1b[41m:(\x1b[0m]FAILED Please Input Passw0rld!");
            PA_.truncate(PA_.len()-1);
            let mut PA_1 = read_fileF("passworld.txt".to_string(),PA_.to_string()).expect("[\x1b[41m:(\x1b[0m] '\x1b[34m Please guide the file correctly, such as '/path/path1/pass.txt', enter the full path or change the file to 'passworld.txt' and put it in the same path as the program, or in the terminal path!] 请正确引导文件,例如'/path/path1/pass.txt'这个输入全路径或者将文件更改为'passworld.txt'然后放在和程序同一路径下,或者终端路径下! [\x1b[42m:)\x1b[0m]  ");
                if num_ == -1 {
                    if num_2_p == -1{
                        _func(20,FNP_C,20, PA_1);
                    }else{
                        _func(20,FNP_C,num_2_p.try_into().unwrap(),PA_1);
                    }
                }else{
                    if num_2_p == -1{
                        _func(num_.try_into().unwrap(),FNP_C,20,PA_1);
                    }else{
                        _func(num_.try_into().unwrap(),FNP_C,num_2_p.try_into().unwrap(),PA_1);
                    }
                }
            }
            else if input.trim() == "N"
            {
                return;
            }
            else
            {
                return;
            }
    }else if args.len() > 1{
        //fn _func(time_Sleep: u64, FNP: String, count_1: i32, _PASSWORLD_1: String)
        for (i, arg) in args.iter().enumerate() 
        {
            if arg == &"--delay"||arg == &"d"{
                let num: Result<i64, _> = args[i+1].clone().trim().parse();
                let num_ = num.unwrap_or(-1);
                _func(num_.try_into().unwrap(), "".to_string(), 8, "".to_string());
            }
            if arg == &"--length"||arg == &"len"{
                let num: Result<i64, _> = args[i+1].clone().trim().parse();
                let num_ = num.unwrap_or(-1);
                _func(20, "".to_string(), num_.try_into().unwrap(), "".to_string());
            }
            if arg == &"--FilePath"||arg == &"--FP"||arg ==&"file_path"{
                _func(20, args[i+1].clone(), 8, "".to_string());
            }
            if arg == &"--passworld"||arg == &"pass"{
                _func(20, "".to_string(), 0,args[i+1].clone());
            }
            /* 
            if arg == &"--passworld"||arg == &"pass" && arg == &"--FilePath"||arg == &"--FP"||arg ==&"file_path" && arg == &"--length"||arg == &"len" && arg == &"--delay"||arg == &"d"{
                func_();
            }
            */
            if arg == &"--version"||arg == &"v"{
                println!("{}",String::from_utf8(decode("CiAgICAbWzMzbShWZXJzaW9uKRtbMG0KICAgICAgICAgICAgICAgIE5vIGZ1Y2tpbmcgVmVyc2lvbhtbNDFtOikbWzBtLgogICAg").unwrap()).unwrap());
            }
            if arg == &"--help"||arg == &"h"
            {
                println!("{}", String::from_utf8(decode("CiAgICBVc2FnZToKICAgICAgICAtLXZlcnNpb24gICAgICAgICAgICAgICAgIERvIHlvdSB3YW50IHRvIHNlZSB0aGUgdmVyc2lvbj8gTXkgcHJvZ3JhbSBkb2VzIG5vdCBoYXZlIHRoZSBjb25jZXB0IG9mIHZlcnNpb24sIGlmIHlvdSB3YW50IHRvIHNlZSBpdCwgcGxlYXNlIHR5cGUgdGhpcyBjb21tYW5kLiAgCiAgICAgICAgLS1oZWxwICBoICAgICAgICAgICAgICAgICBJZiB5b3UgZnVja2luZyB3YW50IG1vcmUgaGVscCB0eXBlIHRoaXMgZ2V0IGNvbW1hbmQgYXJndW1lbnRzLgogICAgICAgIC0tZGVsYXkgIGQgICAgICAgICAgICAgICAgVG9vIGZhc3Q/IFdhbnQgc29tZXRoaW5nIGEgbGl0dGxlIHNhZmVyPyBUaGVuIHRyYWRlIHNvbWUgcGVyZm9ybWFuY2UgZm9yIHNhZmV0eT8gU2V0IHRoZSBkZWxheS4KICAgICAgICAtLWxlbmd0aCBsZW4gICAgICAgICAgICAgIFdoYXQ/IERvIHlvdSB0aGluayB0aGUgcGFzc3dvcmQgaXMgdG9vIGxvbmc/IFRoZW4gc2V0IHRoaXMhCiAgICAgICAgLS1GaWxlUGF0aChGUCkgZmlsZV9wYXRoICBEbyB5b3UgdGhpbmsgaXQgaXMgc2xvdz8gVGhlbiBjcmVhdGUgYSBmaWxlIHRvIHNldCBjb21tb25seSB1c2VkIGNoYXJhY3RlcnMhCiAgICAgICAgLS1wYXNzd29ybGQgcGFzcyAgICAgICAgICB0aGUgcGFzc3dvcmxkIGZpbGUoZGVmYXVsdCBmaWxlIGlzIGN1cnJlbnQgcGF0aCAncGFzc3dvcmxkLnR4dCcpCiAgICA=").unwrap()).unwrap());
                std::process::exit(0);
            }
            if arg == &"--test"||arg == &"t"
            {
                let a = Duration::from_millis(400);
                println!("{:?}", args[i+1].clone());
            }
        }
    }
}
