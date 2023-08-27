use std::env;

/**
 * @Author: 白银
 * @Date: 2022-09-02 16:48:01
 * @LastEditTime: 2022-09-02 17:05:50
 * @LastEditors: 白银
 * @Description: 命令行参数可使用 std::env::args 进行接收，这将返回一个迭代器，该迭代器会对 每个参数举出一个字符串 https://rustwiki.org/zh-CN/rust-by-example/std_misc/arg.html
 */

fn main() {
    let args: Vec<String> = env::args().collect();

    // 第一个参数是调用本程序的路径
    println!("My path is {}.", args[0]);

    // 其余的参数是被传递给程序的命令行参数。
    // 请这样调用程序：
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}