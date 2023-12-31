use std::fs::File;
use std::io::Write;
use std::path::Path;

/**
 * @Author: 白银
 * @Date: 2022-09-02 15:44:45
 * @LastEditTime: 2022-09-02 15:55:57
 * @LastEditors: 白银
 * @Description: create 静态方法以只写模式（write-only mode）打开一个文件 https://rustwiki.org/zh-CN/rust-by-example/std_misc/file/create.html
 */

static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();
    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {:?}", display, why),
        Ok(file) => file,
    };

    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {:?}", display, why)
        }
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
