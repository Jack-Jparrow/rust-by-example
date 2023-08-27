/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-24 20:52:02
 * @LastEditTime: 2022-08-24 20:53:20
 * @LastEditors: Jack Jparrow
 * @Description: 模块可以分配到文件/目录的层次结构中。让我们将《可见性》一节中 的例子的代码拆分到多个文件中 https://rustwiki.org/zh-CN/rust-by-example/mod/split.html
 */

#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
