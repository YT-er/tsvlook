use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了文件路径参数
    if args.len() < 2 {
        println!("请提供文件路径参数");
        return;
    }
    // 获取文件路径参数
    let file_path = &args[1];

    // 打开文件
    let mut file = File::open(file_path).expect("无法打开文件");

    // 创建一个可变字符串来存储文件内容
    let mut contents = Vec::new();

    // 读取文件内容到字符串中
    file.read_to_end(&mut contents).expect("无法读取文件内容");

    // 打印文件内容
    // println!("{}", contents);
    io::stdout().write_all(&contents).expect("无法写入到标准输出");
}
