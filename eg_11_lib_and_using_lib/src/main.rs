/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-24 21:03:33
 * @LastEditTime: 2022-08-24 21:06:30
 * @LastEditors: Jack Jparrow
 * @Description: 要将一个 crate 链接到上节新建的库，可以使用 rustc 的 --extern 选项。然后将所有的物件导入到与库名相同的模块下。此模块的操作通常与任何其他模块相同 https://rustwiki.org/zh-CN/rust-by-example/crates/using_lib.html
 */

// extern crate rary; // 在 Rust 2015 版或更早版本需要这个导入语句
fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}
