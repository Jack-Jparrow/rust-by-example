use std::path::Path;

/**
 * @Author: 白银
 * @Date: 2022-09-02 15:34:16
 * @LastEditTime: 2022-09-02 15:43:39
 * @LastEditors: 白银
 * @Description: Path 结构体代表了底层文件系统的文件路径 https://rustwiki.org/zh-CN/rust-by-example/std_misc/path.html
 */

fn main() {
    // 从 `&'static str` 创建一个 `Path`
    let path = Path::new(".");
    // `display` 方法返回一个可显示（showable）的结构体
    let _display = path.display();
    // `join` 使用操作系统特定的分隔符来合并路径到一个字节容器，并返回新的路径
    let new_path = path.join("a").join("b");

    // 将路径转换成一个字符串切片
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
