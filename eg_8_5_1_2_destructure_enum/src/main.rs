/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 21:05:34
 * @LastEditTime: 2022-08-22 21:16:29
 * @LastEditors: Jack Jparrow
 * @Description: 解构 enum https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/destructuring/destructure_enum.html
 */

#[allow(dead_code)]

enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    // 可以使用 `match` 来解构 `enum`
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),

        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}