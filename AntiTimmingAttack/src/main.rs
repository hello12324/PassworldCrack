use std::str::FromStr;

fn xor(bytes1: &[u8], bytes2: &[u8]) {
    let mut result = Vec::new();
    //for (b1, b2) in bytes1.iter().zip(bytes2) {
    //    result.push(b1 ^ b2);
    //}
    result.push(bytes1 ^ bytes2);
    print!("{:?}", result);
}

fn _SAFE_EQU(a: String, b: String) -> bool{
    if a.len() == b.len() {
        let mut pass = 0;
        let mut status = 0;
        loop{
            if 0 < a.len() {
                //pass = xor(a.as_bytes(), b.as_bytes());
                break;
            }
        }
        return pass == 0;
    }else{
        return false;
    }
}
fn main(){
    let str1 = "abc".as_bytes();
    let str2 = "abc".as_bytes();
    xor(str1, str2);
    //let result_str = String::from_utf8(result).unwrap();
    //println!("{}", result_str);
}