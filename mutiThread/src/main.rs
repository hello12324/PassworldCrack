use futures::executor::block_on;
use std::io;

//the test
#[warn(non_snake_case)]
async fn hello_world() {
    let result = hello_cat().await;
    println!("{:?}", result);
}

async fn hello_cat() -> i64 {
    let mut inp = String::new();
    println!("TEST: ");
    io::stdin().read_line(&mut inp).expect(&"fuck");
    let number: Result<i64, _> = inp.trim().parse();
    let number_ = number.unwrap_or(-1);
    return number_;
}

fn main() {
    let future = hello_world();
    block_on(future);
}
