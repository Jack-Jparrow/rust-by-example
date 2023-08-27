/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-11 20:51:13
 * @LastEditTime: 2022-08-11 21:39:05
 * @LastEditors: Jack Jparrow
 * @Description: 美化打印 https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_debug.html
 */

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
