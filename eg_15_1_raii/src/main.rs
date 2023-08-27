/**
 * @Author: 白银
 * @Date: 2022-08-29 19:27:13
 * @LastEditTime: 2022-08-29 19:31:25
 * @LastEditors: 白银
 * @Description: 当 main 函数中的变量离开作用域，自定义的析构函数就会被调用 https://rustwiki.org/zh-CN/rust-by-example/scope/raii.html
 */

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;

    println!("Made a ToDrop!");
}
