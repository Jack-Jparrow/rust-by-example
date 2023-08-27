/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-23 21:13:03
 * @LastEditTime: 2022-08-23 21:17:01
 * @LastEditors: Jack Jparrow
 * @Description: 指明为该结构体实现的是 Fn、FnMut、或 FnOnce 中的哪种 trait，对于约束该结构体的类型而言就已经足够了 https://rustwiki.org/zh-CN/rust-by-example/fn/closures/anonymity.html
 */

// `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`，这和对 `print` 的要求恰好一样
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let x = 7;
    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`
    // 将闭包存储到 `print` 中
    let print = || println!("{}", x);

    apply(print);
}
