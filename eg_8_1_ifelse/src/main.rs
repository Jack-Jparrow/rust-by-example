/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-21 20:03:02
 * @LastEditTime: 2022-08-21 20:09:22
 * @LastEditors: Jack Jparrow
 * @Description: if-else https://rustwiki.org/zh-CN/rust-by-example/flow_control/if_else.html
 */

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is negative", n);
    } else {
        print!("{} is negative", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // 这个表达式返回一个 `i32` 类型
        10 * n
    } else {
        println!(", and is a big number, half the number");

        // 这个表达式也必须返回一个 `i32` 类型
        n / 2
    }; // 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它

    println!("{} -> {}", n, big_n);
}
