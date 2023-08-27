use std::{fs::File, io::Read, path::Path};

/**
 * @Author: 白银
 * @Date: 2022-09-02 15:40:27
 * @LastEditTime: 2022-09-02 15:50:10
 * @LastEditors: 白银
 * @Description: open 静态方法能够以只读模式（read-only mode）打开一个文件 https://rustwiki.org/zh-CN/rust-by-example/std_misc/file/open.html
 */

fn main() {
    // 创建指向所需的文件的路径
    let path = Path::new("hello.txt");
    let display = path.display();
    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {:?}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {:?}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` 离开作用域，并且 `hello.txt` 文件将被关闭
}
