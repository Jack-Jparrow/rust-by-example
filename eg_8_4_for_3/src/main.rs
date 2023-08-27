/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 21:03:52
 * @LastEditTime: 2022-08-21 21:27:17
 * @LastEditors: Jack Jparrow
 * @Description: iter - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用 https://rustwiki.org/zh-CN/rust-by-example/flow_control/for.html
 */

fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
