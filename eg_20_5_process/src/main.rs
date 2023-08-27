use std::process::Command;

/**
 * @Author: 白银
 * @Date: 2022-09-02 15:59:34
 * @LastEditTime: 2022-09-02 16:36:14
 * @LastEditors: 白银
 * @Description: process::Command 结构体是一个进程创建者（process builder） https://rustwiki.org/zh-CN/rust-by-example/std_misc/process.html#%E5%AD%90%E8%BF%9B%E7%A8%8B
 */

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}
