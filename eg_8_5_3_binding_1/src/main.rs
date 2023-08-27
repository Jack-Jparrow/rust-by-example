/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 21:57:46
 * @LastEditTime: 2022-08-22 22:01:43
 * @LastEditors: Jack Jparrow
 * @Description: match 提供了 @ 符号来绑定变量到名称 https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/binding.html
 */

// `age` 函数，返回一个 `u32` 值
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结
        n => println!("I'm an old person of age {:?}", n),
    }
}
