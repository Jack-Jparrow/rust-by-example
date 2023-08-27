/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-25 20:22:51
 * @LastEditTime: 2022-08-25 20:28:24
 * @LastEditors: Jack Jparrow
 * @Description: 条件编译 https://rustwiki.org/zh-CN/rust-by-example/attribute/cfg.html
 */

// 这个函数仅当目标系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// 而这个函数仅当目标系统 **不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");

    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
