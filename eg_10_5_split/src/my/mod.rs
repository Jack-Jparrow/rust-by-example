/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-24 20:51:05
 * @LastEditTime: 2022-08-24 20:56:10
 * @LastEditors: Jack Jparrow
 * @Description: 模块可以分配到文件/目录的层次结构中。让我们将《可见性》一节中 的例子的代码拆分到多个文件中 https://rustwiki.org/zh-CN/rust-by-example/mod/split.html
 */
// 类似地，`mod inaccessible` 和 `mod nested` 将找到 `nested.rs` 和 `inaccessible.rs` 文件，并在它们放到各自的模块中
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    println!("called `my::indirect_access()`, that\n> ");

    private_function();
}
