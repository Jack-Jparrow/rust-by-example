/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 21:04:05
 * @LastEditTime: 2022-08-21 21:31:38
 * @LastEditors: Jack Jparrow
 * @Description: iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改 https://rustwiki.org/zh-CN/rust-by-example/flow_control/for.html
 */

fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
