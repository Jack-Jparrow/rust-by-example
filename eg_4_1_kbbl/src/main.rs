/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-18 21:34:59
 * @LastEditTime: 2022-08-18 21:40:53
 * @LastEditors: Jack Jparrow
 * @Description: 变量绑定默认是不可变的（immutable），但加上 mut 修饰语后变量就可以改变 https://rustwiki.org/zh-CN/rust-by-example/variable_bindings/mut.html
 */

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // 正确代码
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误！
    // _immutable_binding += 1;
}
