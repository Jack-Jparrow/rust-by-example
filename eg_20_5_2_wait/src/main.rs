use std::process::Command;

/**
 * @Author: 白银
 * @Date: 2022-09-02 16:37:17
 * @LastEditTime: 2022-09-02 16:45:57
 * @LastEditors: 白银
 * @Description: 如果你想等待一个 process::Child 完成，就必须调用 Child::wait，这会返回 一个 process::ExitStatus https://rustwiki.org/zh-CN/rust-by-example/std_misc/process/wait.html
 */

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}