use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/**
 * @Author: 白银
 * @Date: 2022-09-02 15:51:29
 * @LastEditTime: 2022-09-02 16:04:37
 * @LastEditors: 白银
 * @Description: File::open 需要一个泛型 AsRef<Path>。这正是 read_lines() 期望的输入 https://rustwiki.org/zh-CN/rust-by-example/std_misc/file/read_lines.html
 */

fn main() {
    // 在生成输出之前，文件主机必须存在于当前路径中
    if let Ok(lines) = read_lines("./hosts") {
        // 使用迭代器，返回一个（可选）字符串
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}
// 输出包裹在 Result 中以允许匹配错误，将迭代器返回给文件行的读取器（Reader）
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}
