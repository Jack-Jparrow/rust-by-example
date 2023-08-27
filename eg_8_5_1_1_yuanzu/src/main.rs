/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 20:58:56
 * @LastEditTime: 2022-08-22 21:03:03
 * @LastEditors: Jack Jparrow
 * @Description: 元组可以在 match 中解构 https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/destructuring/destructure_tuple.html
 */

fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        // match 可以解构一个元组
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        // `..` 可用来忽略元组的其余部分
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // `_` 表示不将值绑定到变量
        _ => println!("It doesn't matter what they are"),
    }
}
