use std::fmt::Debug; // 用于约束的 trait

/**
 * @Author: 白银
 * @Date: 2022-08-30 21:18:21
 * @LastEditTime: 2022-08-30 21:26:58
 * @LastEditors: 白银
 * @Description: 生命周期（它们本身就是泛型）也可以使用约束 https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/lifetime_bounds.html
 */

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` 包含一个指向泛型类型 `T` 的引用，其中 `T` 拥有一个未知的生命周期 `'a`。`T` 拥有生命周期限制， `T` 中的任何*引用*都必须比 `'a` 活得更长。另外 `Ref` 的生命周期也不能超出 `'a`

// 一个泛型函数，使用 `Debug` trait 来打印内容
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

// 这里接受一个指向 `T` 的引用，其中 `T` 实现了 `Debug` trait，并且在 `T` 中的所有*引用*都必须比 `'a'` 存活时间更长。另外，`'a` 也要比函数活得更长
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
