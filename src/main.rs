
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("在{}", filename);
    println!("寻找{}中...", query);

}