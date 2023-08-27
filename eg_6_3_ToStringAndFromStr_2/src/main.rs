/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-20 21:49:46
 * @LastEditTime: 2022-08-20 21:53:30
 * @LastEditors: Jack Jparrow
 * @Description: 一个实现 ToString 的例子 https://rustwiki.org/zh-CN/rust-by-example/conversion/string.html
 */

struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
