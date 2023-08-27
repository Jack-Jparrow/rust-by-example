/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 22:32:29
 * @LastEditTime: 2022-08-22 22:35:41
 * @LastEditors: Jack Jparrow
 * @Description: 和 if let 类似，while let 也可以把别扭的 match 改写得好看一些。考虑下面这段使 i 不断增加的代码： https://rustwiki.org/zh-CN/rust-by-example/flow_control/while_let.html
 */

fn main() {
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    loop {
        match optional {
            // 如果 `optional` 解构成功，就执行下面语句块
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ 需要三层缩进！
            }

            // 当解构失败时退出循环：
            _ => {
                break;
            }
        }
    }
}
