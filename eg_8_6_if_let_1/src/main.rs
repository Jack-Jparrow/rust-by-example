/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 22:12:18
 * @LastEditTime: 2022-08-22 22:16:20
 * @LastEditors: Jack Jparrow
 * @Description: 在一些场合下，用 match 匹配枚举类型并不优雅 https://rustwiki.org/zh-CN/rust-by-example/flow_control/if_let.html
 */

fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        }
        _ => {}
    }
}
