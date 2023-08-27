/**
 * @Author: 白银
 * @Date: 2022-09-01 14:05:34
 * @LastEditTime: 2022-09-01 14:26:37
 * @LastEditors: 白银
 * @Description: 要定义一套小的计算器 API，可以传给它表达式，它会把结果打印到控制台上 https://rustwiki.org/zh-CN/rust-by-example/macros/dsl.html
 */

macro_rules! calculate {
    (eval $e: expr) => {
        {
            let val: usize = $e;// 强制类型为整型

            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate!(
        eval 1 + 2// 看到了吧，`eval` 可并不是 Rust 的关键字！
    );

    calculate!(
        eval (1 + 2) * (3 / 4)
    )
}