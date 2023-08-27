/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 21:03:56
 * @LastEditTime: 2022-08-21 21:29:58
 * @LastEditors: Jack Jparrow
 * @Description: into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了 https://rustwiki.org/zh-CN/rust-by-example/flow_control/for.html
 */

fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
