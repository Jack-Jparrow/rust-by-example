/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-18 21:52:12
 * @LastEditTime: 2022-08-18 21:59:29
 * @LastEditors: Jack Jparrow
 * @Description: 可以先声明（declare）变量绑定，后面才将它们初始化（initialize） https://rustwiki.org/zh-CN/rust-by-example/variable_bindings/declare.html
 */

fn main() {
    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化一个绑定
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // 报错！使用了未初始化的绑定
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}