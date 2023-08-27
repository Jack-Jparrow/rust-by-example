use std::fmt::Display;

/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-20 21:41:31
 * @LastEditTime: 2022-08-20 21:48:16
 * @LastEditors: Jack Jparrow
 * @Description: 要把任何类型转换成 String，只需要实现fmt::Display trait，它会自动提供 ToString，并且还可以用来打印类型 https://rustwiki.org/zh-CN/rust-by-example/conversion/string.html
 */

struct Circle {
    radius: i32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
