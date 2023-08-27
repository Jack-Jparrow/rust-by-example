/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-17 22:25:48
 * @LastEditTime: 2022-08-17 22:31:05
 * @LastEditors: Jack Jparrow
 * @Description: 常量 https://rustwiki.org/zh-CN/rust-by-example/custom_types/constants.html
 */

// 全局变量是在所有其他作用域之外声明的
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 会报错！不能修改一个 `const` 常量
    // THRESHOLD = 5;
}
