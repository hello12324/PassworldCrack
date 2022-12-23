use std::{thread, time};
use std::io;

fn _func(time_Sleep: u64)
{
    let time_sleep = time::Duration::from_millis(time_Sleep);
    let strings = format!("{}{}","abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_-+~`|\\{\\}[]:;'<,>.?/",'"');
    let mut length_ = 0;
    for length in strings.chars() 
    {
        length_ += 1;
    }
    println!("string length is \x1b[32m {}\x1b[0m",length_);
    thread::sleep(time::Duration::from_millis(1000));
    //println!("{}", strings);
    let mut num = 0;
    for i0 in 1..=94
    {
        if i0 == 1
        {
            for c0 in strings.chars()
            {
                num +=1;
                let mut passworld = c0.to_string();
                println!("{} <- {}\t",num,passworld);thread::sleep(time_sleep);
            }
        }else if i0 == 2
        {
            for c0 in strings.chars()
            {
                for c1 in strings.chars()
                {
                    num +=1;
                    let mut passworld = c0.to_string()+&c1.to_string();
                    println!("{} <- {}\t",num,passworld);thread::sleep(time_sleep);
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
                            let mut passworld = c0.to_string()+&c1.to_string()+&c2.to_string();
                            num +=1;
                            println!("{} <- {}\t",num,passworld);thread::sleep(time_sleep);
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
                                let mut passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string();
                                num+=1;
                                println!("{} <- {}\t", num, passworld);thread::sleep(time_sleep);
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
                                
                                let mut passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string();
                                num+=1;
                                println!("{} <- {}\t", num, passworld);thread::sleep(time_sleep);
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

                                    let mut passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string();
                                    num+=1;
                                    println!("{} <- {}\t", num, passworld);thread::sleep(time_sleep);
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
                                        
                                    let mut passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string();
                                    num+=1;
                                    println!("{} <- {}\t", num, passworld);thread::sleep(time_sleep);
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
                                            let mut passworld = c0.to_string()+&c1.to_string()+&c2.to_string()+&c3.to_string()+&c4.to_string()+&c5.to_string()+&c6.to_string()+&c7.to_string();
                                            num+=1;
                                            println!("{} <- {}\t", num, passworld);thread::sleep(time_sleep);

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

fn main() {
    println!("Are You Sure continue?(Yes or No Default:Y)>");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed");
    println!("Delay (default 20)>");
    let mut time_ = String::new();
    io::stdin().read_line(&mut time_).expect("Failed");
    let num: Result<i64, _> = time_.trim().parse();
    let num_ = num.unwrap_or(0);
    if input.trim() == "" || input.trim() == "Y"
    {
        if num_ == num_ {
            _func(20);
        }else{
            _func(num_.try_into().unwrap());
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