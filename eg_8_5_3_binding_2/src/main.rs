/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 22:02:52
 * @LastEditTime: 2022-08-22 22:10:52
 * @LastEditors: Jack Jparrow
 * @Description: 可以使用绑定来“解构” enum 变体，例如 Option https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/binding.html
 */

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    match some_number() {
        // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 匹配任意其他数字
        Some(n) => println!("Not interesting... {}", n),
        // 匹配任意其他值（`None` 可变类型
        _ => (),
    }
}
