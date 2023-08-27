/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-24 20:53:34
 * @LastEditTime: 2022-08-24 20:54:38
 * @LastEditors: Jack Jparrow
 * @Description: 模块可以分配到文件/目录的层次结构中。让我们将《可见性》一节中 的例子的代码拆分到多个文件中 https://rustwiki.org/zh-CN/rust-by-example/mod/split.html
 */

pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
