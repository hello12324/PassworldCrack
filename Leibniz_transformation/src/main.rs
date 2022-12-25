use std::{
    f64::consts::PI,
};
use base64::{
    decode,
};
fn _func()
{
    let n = 8;
    let xs = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let ys = [1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0];

    let mut out = vec![0.0; n];
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            let angle = 2.0 * PI * (j as f64) * (i as f64) / (n as f64);
            sum += xs[j] * angle.cos() - ys[j] * angle.sin();
        }
        out[i] = sum;
    }

    println!("\x1b[34m{:?}\x1b[0m", out);
}
fn main() {
    println!("{}", String::from_utf8(decode("CiAgICAgX19fX19fX19fX19fX19fX19fX19fX18KICAgIHxMZWlibml6ICAgG1szMm06KBtbMG0gICAgICAgICAgIHwKICAgIHwgICAbWzMxbTopG1swbSAgdHJhbnNmb3JtYXRpb24gIHwKICAgICAtLS0tLS0tLS0tLS0tLS0tLS0tLS0tLQogICAgICAgICAgICAbWzMxbU1lcnJ5G1swbSAbWzMybUNocmlzdG1hcyEbWzBtCiAgICAgICAgICAgICAgICBtYWtlIGJ5IGdldW1ibyAgICAKCg==").unwrap()).unwrap());
    _func();
}

