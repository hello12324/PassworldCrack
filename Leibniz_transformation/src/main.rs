//      __         _ __          _          __                        ____                           __  _           
//     / /   ___  (_) /_  ____  (_)___     / /__________ _____  _____/ __/___  _________ ___  ____ _/ /_(_)___  ____ 
//    / /   / _ \/ / __ \/ __ \/ /_  /    / __/ ___/ __ `/ __ \/ ___/ /_/ __ \/ ___/ __ `__ \/ __ `/ __/ / __ \/ __ \
//   / /___/  __/ / /_/ / / / / / / /_   / /_/ /  / /_/ / / / (__  ) __/ /_/ / /  / / / / / / /_/ / /_/ / /_/ / / / /
//  / /_____/\___/_/_.___/_/ /_/_/ /___/   \__/_/   \__,_/_/ /_/____/_/  \____/_/  /_/ /_/ /_/\__,_/\__/_/\____/_/ /_/ 
//                                                                                                                                                           
//                                                                                                   
// _____________________________________________________________________________________________________________________________
//|The Leibniz transformation is a mathematical tool that is often used to solve ordinary differential and integral equations.  |
//|It can convert the time-domain function into a frequency-domain function,                                                    |
//|    making the problem easier to solve.                                                                                      |
//|                                                                                                                             |
//|In the computer field, the Leibniz transformation has a wide range of applications. For example, it can be used for:         |
//|                                                                                                                             |
//| -Image processing: Leibniz transform can be used for image sharpening, denoising, image segmentation and other operations.  |
//|                                                                                                                             |
//| -Audio processing: Leibniz transforms can be used in the analysis, processing and adjustment of audio signals.              |
//|                                                                                                                             |
//| -Data Compression: Leibniz transformations can be used to compress data to a smaller size, saving storage space.            |
//|                                                                                                                             |
//|In general,                                                                                                                  |
//|    Leibniz transformation plays an important role in the computer field,                                                    |
//|            providing us with a powerful tool to solve various mathematical problems.                                        |        
//|                                                                                                                             |
//| 2022-12-26 <My emal>testsendkfotesycike@gmail.com                                                                           |
//| 	Merry Christmas!                                                                                                        |
//|                                                              <Make by GeumBo>                                               |
// -----------------------------------------------------------------------------------------------------------------------------
//
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

