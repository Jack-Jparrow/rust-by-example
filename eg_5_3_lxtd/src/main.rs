/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-1&9 21:40:51
 * @LastEditTime: 2022-08-19 21:43:42
 * @LastEditors: Jack Jparrow
 * @Description: Rust 的类型推断引擎是很聪明的，它不只是在初始化时看看右值（r-value）的 类型而已，它还会考察变量之后会怎样使用，借此推断类型 https://rustwiki.org/zh-CN/rust-by-example/types/inference.html
 */

fn main() {
    // 因为有类型说明，编译器知道 `elem` 的类型是 u8
    let elem = 5u8;
    // 创建一个空向量（vector，即不定长的，可以增长的数组）
    let mut vec = Vec::new();
    // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）

    // 在向量中插入 `elem`
    vec.push(elem);
    // 现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）

    println!("{:?}", vec);
}
