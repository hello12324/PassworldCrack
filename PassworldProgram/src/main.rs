use std::io;
//use crossterm::*;
use std::fs;
use std::env;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use serde::__private::de::Content;
use std::io::ErrorKind;

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
    loop{
        let content = fs::read_to_string(path);
        match content {
            Ok(t) => {
                Ok(t)
            }
            Err(a) => {
                if a.kind() == ErrorKind::NotFound{
                    write_file("passworld", "d2b99f358788d2b8f2ec4a180ca951252806708f2139a9693d52ff14cc32d1fe")
                }else {
                    Err(a)
                }
            }
        }
    }
}

fn write_file(fileName: String, Contents: String) -> String
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
    println!("
      ___________ 
     | Passw0rd:(|
     | :) Pr0gram|
      -----------
    ");
    _func();
    //return;
}

