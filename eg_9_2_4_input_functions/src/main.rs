/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-23 21:17:28
 * @LastEditTime: 2022-08-23 21:20:18
 * @LastEditors: Jack Jparrow
 * @Description: 如果你声明一个接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为其参数 https://rustwiki.org/zh-CN/rust-by-example/fn/closures/input_functions.html
 */

// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）
fn function() {
    println!("I'm a function!");
}

fn main() {
    // 定义一个满足 `Fn` 约束的闭包
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}