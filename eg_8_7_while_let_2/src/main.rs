/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 22:36:47
 * @LastEditTime: 2022-08-22 22:40:03
 * @LastEditors: Jack Jparrow
 * @Description: 使用 while let 可以使这段代码变得更加优雅 https://rustwiki.org/zh-CN/rust-by-example/flow_control/while_let.html
 */

fn main() {
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就执行语句块（`{}`）。否则就 `break`
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况
    }
    // ^ `if let` 有可选的 `else`/`else if` 分句，
    // 而 `while let` 没有
}
