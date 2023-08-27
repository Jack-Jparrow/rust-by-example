/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-24 20:47:26
 * @LastEditTime: 2022-08-24 20:57:12
 * @LastEditors: Jack Jparrow
 * @Description: 模块可以分配到文件/目录的层次结构中。让我们将《可见性》一节中 的例子的代码拆分到多个文件中 https://rustwiki.org/zh-CN/rust-by-example/mod/split.html
 */
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容放到此作用域中一个名为 `my` 的模块里面
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
