use std::fs::{read_to_string};
use rand::{thread_rng, Rng};
use std::{
    time::Duration,
    thread, 
    time, 
    fs,
    io,
    env,
    io::ErrorKind,
    time::{
        Instant
    }
};
use crypto::{
    sha2::Sha256,
    digest::Digest
};

//内部的代码可能稍微有点不是很人性化:).


fn hash_encryt(hash_value: String) -> String
{
    let mut hasher = Sha256::new();
    let _content = String::from(hash_value);
    hasher.input_str(&_content);
    return hasher.result_str()
}
fn output_style(_passworld: String, _time_sleep: Duration)
{
    
    let _1passworld = _passworld.clone();
    let mut rng = thread_rng();
    let a = rng.gen_range(31, 37);
    println!("[\x1b[32m:)\x1b[0m \x1b[34mTEST OUTPUT\x1b[0m] [\x1b[{fuck}m{}\x1b[0m]",_passworld,fuck=a);thread::sleep(_time_sleep);
    let start_time = Instant::now();
    let passworld_ = passworldTEST("abcd".to_string(), _passworld);
    let stop_time = start_time.elapsed();
    if passworld_ == true{
        println!("[\x1b[32m:)\x1b[0m]Passworld Success, Passworld is \x1b[34m{}\x1b[0m, time => {}", &_1passworld,stop_time.as_millis());
        std::process::exit(0);
    }
    
}
fn passworldTEST(pasphrase: String, Enum: String) -> bool
{
    let mut pass = 0;
    if Enum == pasphrase {
        pass = 1;
    }else{
        pass = 0;
    }
    if pass == 1{
        return true;
    }else if pass == 0{
        return false;
    }else{
        return false;
    }
} 
fn _file(contents: String)
{
    let current_dir = env::current_dir().unwrap();
    fs::write(current_dir.join("Passworldenum.txt"), contents);

}
fn read_file(FP: String) -> Result<String, std::io::Error> 
{
    let current_dir_1 = env::current_dir().unwrap();
    let current_file = current_dir_1.join("DefaultPassworld.txt");
    let content = fs::read_to_string(FP.trim());
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

fn _func(time_Sleep: u64, FNP: String, count_1: i32)
{
    let time_sleep = time::Duration::from_millis(time_Sleep);
    //let strings = format!("{}{}","abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\\{\\}[]:;'<,>.?/",'"');
    let strings = read_file(FNP.to_string()).expect("[\x1b[41m:(\x1b[0m] '\x1b[34mDefaultPassworld.txt\x1b[0m' file is NotFound, please create this file on program dir path and add content 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\\{//}[]:;'<,>.?/'{}' or add your want add custom content[没有找到'\x1b[34mDefaultPassworld.txt\x1b[0m',请手动创建文件到程序的目录下并添加内容'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\\{//}[]:;'<,>.?/'{}'或者添加自定义内容] [\x1b[42m:)\x1b[0m]  ");
    println!("[\x1b[32m:)\x1b[0m] file content is {}",strings);
    let mut length_ = 0;
    for length in strings.chars() 
    {
        length_ += 1;
    }
    println!("[\x1b[32m:)\x1b[0m] string length is \x1b[32m {}\x1b[0m bytes",length_);
    thread::sleep(time::Duration::from_millis(1000));
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
                output_style(passworld, time_sleep);
            }
        }else if i0 == 2
        {
            for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    num +=1;
                    let passworld = c0.to_string()+&c1.to_string();
                    output_style(passworld, time_sleep);
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
                            output_style(passworld, time_sleep);
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
                                output_style(passworld, time_sleep);
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
                                output_style(passworld, time_sleep);
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
                                    output_style(passworld, time_sleep);
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
                                    output_style(passworld, time_sleep);
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
                                            output_style(passworld, time_sleep);

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
                                                output_style(passworld, time_sleep);
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
                                                    output_style(passworld, time_sleep);
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
                                                        output_style(passworld, time_sleep);
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
                                                            output_style(passworld, time_sleep);
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
                                                                output_style(passworld, time_sleep);
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
                                                                    output_style(passworld, time_sleep);
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
                                                                        output_style(passworld, time_sleep);
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
                                                                            output_style(passworld, time_sleep);
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
                                                                                output_style(passworld, time_sleep);
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
                                                                                    output_style(passworld, time_sleep);
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
                                                                                        output_style(passworld, time_sleep);
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
                                                                                            output_style(passworld, time_sleep);
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

fn main() {
    println!("
     _____________
    |Passw0rld    |
    | \x1b[32m:(\x1b[0mCrack  \x1b[41m:)\x1b[0m |
     -------------
            make by geumbo
    ");
    println!("[\x1b[32m:)\x1b[0m] Are You Sure continue?(Yes or No Default:Y)[是否继续 默认继续]");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("[\x1b[41m:(\x1b[0m]failed");
    if input.trim() == "" || input.trim() == "Y"
    {
    println!("[\x1b[32m:)\x1b[0m] Passworld length(Default is 20)[密码的长度 默认20]");
    let mut count_2 = String::new();
    io::stdin().read_line(&mut count_2).expect("[\x1b[41m:(\x1b[0m]Failed");
    let num_2: Result<i64, _> = count_2.trim().parse();
    let num_2_p = num_2.unwrap_or(-1);
    println!("[\x1b[32m:)\x1b[0m] Delay(default 400)[延迟, 默认为400ms]");
    let mut time_ = String::new();
    io::stdin().read_line(&mut time_).expect("[\x1b[41m:(\x1b[0m]Failed");
    let num: Result<i64, _> = time_.trim().parse();
    let num_ = num.unwrap_or(-1);
    println!("[\x1b[32m:)\x1b[0m] custom file[自定义文件]");
    let mut FNP_C = String::new();
    io::stdin().read_line(&mut FNP_C).expect("[\x1b[41m:(\x1b[0m]FAILED");
    //println!("{}",FNP_C);
        if num_ == -1 {
            if num_2_p == -1{
                _func(0,FNP_C,20);
            }else{
                _func(0,FNP_C,num_2_p.try_into().unwrap());
            }
        }else{
            if num_2_p == -1{
                _func(num_.try_into().unwrap(),FNP_C,20);
            }else{
                _func(num_.try_into().unwrap(),FNP_C,num_2_p.try_into().unwrap());
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
}