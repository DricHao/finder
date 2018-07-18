
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("在{};", filename);
    println!("寻找'{}'中...", query);

    let mut f = File::open(filename).expect("该文件没有找到");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("wrong!");

    println!("文件内容是： {}", contents);

}