/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-17 21:52:35
 * @LastEditTime: 2022-08-17 21:59:57
 * @LastEditors: Jack Jparrow
 * @Description: enum 也可以像 C 语言风格的枚举类型那样使用 https://rustwiki.org/zh-CN/rust-by-example/custom_types/enum/c_like.html
 */

#[allow(dead_code)]
// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

#[allow(dead_code)]
// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enum` 可以转成整型
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}