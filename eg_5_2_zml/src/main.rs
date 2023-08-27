use std::mem::size_of_val;

/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-19 21:35:01
 * @LastEditTime: 2022-08-19 21:38:43
 * @LastEditors: Jack Jparrow
 * @Description: 对数值字面量，只要把类型作为后缀加上去，就完成了类型说明；无后缀的数值字面量，其类型取决于怎样使用它们。如果没有限制，编译器会对整数使用 i32，对浮点数使用 f64 https://rustwiki.org/zh-CN/rust-by-example/types/literals.html
 */

fn main() {
    // 带后缀的字面量，其类型在初始化时已经知道了
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    // 无后缀的字面量，其类型取决于如何使用它们
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", size_of_val(&x));
    println!("size of `y` in bytes: {}", size_of_val(&y));
    println!("size of `z` in bytes: {}", size_of_val(&z));
    println!("size of `i` in bytes: {}", size_of_val(&i));
    println!("size of `f` in bytes: {}", size_of_val(&f));
}