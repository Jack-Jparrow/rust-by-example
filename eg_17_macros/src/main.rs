/**
 * @Author: 白银
 * @Date: 2022-09-01 13:37:23
 * @LastEditTime: 2022-09-01 13:42:48
 * @LastEditors: 白银
 * @Description: 宏是通过 macro_rules! 宏来创建的 https://rustwiki.org/zh-CN/rust-by-example/macros.html
 */

// 这是一个简单的宏，名为 `say_hello`
macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数
    () => {
        // 此宏将会展开成这个代码块里面的内容
        println!("Hello!");
    };
}

fn main() {
    // 这个调用将会展开成 `println("Hello");`!
    say_hello!();
}
