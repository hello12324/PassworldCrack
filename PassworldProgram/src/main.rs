//
//      ____  ___   ___________       ______  ____  __    ____       ____  ____  ____  __________  ___    __  ___
//     / __ \/   | / ___/ ___/ |     / / __ \/ __ \/ /   / __ \     / __ \/ __ \/ __ \/ ____/ __ \/   |  /  |/  /
//    / /_/ / /| | \__ \\__ \| | /| / / / / / /_/ / /   / / / /    / /_/ / /_/ / / / / / __/ /_/ / /| | / /|_/ / 
//   / ____/ ___ |___/ /__/ /| |/ |/ / /_/ / _, _/ /___/ /_/ /    / ____/ _, _/ /_/ / /_/ / _, _/ ___ |/ /  / /  
//  /_/   /_/  |_/____/____/ |__/|__/\____/_/ |_/_____/_____/____/_/   /_/ |_|\____/\____/_/ |_/_/  |_/_/  /_/   
//                                                         /_____/                                                 
//                                                                                                   
// _____________________________________________________________________________________________
//| A simple program with a password needs to enter the password to unlock it.                  |
//| The password is encrypted by hash and needs to be cracked by brute force.                   |
//|  How do you crack it? What is the password?                                                 |
//|                                                                                             |
//| 2022-12-26 <My emal>testsendkfotesycike@gmail.com                                           |
//| 	Merry Christmas!                                                                        |
//|                                                              <Make by GeumBo>               |
// ---------------------------------------------------------------------------------------------
//
//use crossterm::*;
use std::{
    fs,
    env,
    io
};
use crypto::{
    sha2::Sha256,
    digest::Digest
};
use base64::{
    encode,
    decode
};
fn hash_encryt(hash_value: String) -> String
{
    let mut hasher = Sha256::new();
    let _content = String::from(hash_value);
    hasher.input_str(&_content);
    return hasher.result_str()
}
fn passphrase() -> String
{ 
    let current_dir = env::current_dir().unwrap();
    let path = current_dir.join("passworld.txt");
    let content = fs::read_to_string(path).expect("Not Found 'passworld.txt' file, please create this file and add hash passworld content[????????????'passworld.txt'??????,?????????????????????????????????hash????????????]");
    return content;
}

fn write_file(fileName: String, Contents: String)
{
    let current_dir = env::current_dir().unwrap();
    fs::write(current_dir.join(fileName), Contents);
}
fn _func()
{
    loop{
        let mut a = String::new();
        println!("[Please Input Value]");
        io::stdin().read_line(&mut a).expect("read line Error");
        let mut pass = 0;
        let passphrase = passphrase();
        let test = a.trim();
        let hash_encoding = hash_encryt(test.to_string());
        println!("your inputValue to HASH is a  -> {}",hash_encoding);
        if hash_encoding == passphrase.trim(){
            println!("\nhey, bingo! Welcome to run this Program, :)!");
            pass = 1;
        }else{
            println!("\n[\x1b[31mx\x1b[0m]Sorry?, your passphrase is failed, Please retry!");
            pass = 0;
        }
    
        if pass == 1{
            success();
            break;
        } else if pass == 0{
            failure();
            //std::process::exit(1);
        }
        
        //println!("lenght -> {}, content -> {}, pass value -> {}",a.len(), a.to_string(), pass);
    }
}
fn success()
{
    println!("
\x1b[34m+++\x1b[0m \x1b[32mbingo!, what?, your success?, Fuckyou!\x1b[0m \x1b[34m+++\x1b[0m");
}
fn failure()
{
    //println!("{}",logo);
    println!("
     \x1b[31m--- FUCKY0U ---- 
    \x1b[0m");
    //std::process::exit(1);
}

fn main() {
    println!("{}", String::from_utf8(decode("CiAgICBfX19fX19fX19fX18KICAgfCBwYXNzdzBybGQbWzMxbTopG1swbXwKICAgfCAgG1szMm06KBtbMG1QcjBncmFtIHwKICAgIC0tLS0tLS0tLS0tLQogICAgICAgIBtbMzFtTWVycnkbWzBtIBtbMzJtQ2hyaXN0bWFzIRtbMG0KICAgICAgICAgICAgbWFrZSBieSBnZXVtYm8K").unwrap()).unwrap());
    _func();
    //return;
}

