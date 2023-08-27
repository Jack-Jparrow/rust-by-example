/**
 * @Author: 白银
 * @Date: 2022-08-30 21:21:33
 * @LastEditTime: 2022-08-30 21:31:36
 * @LastEditors: 白银
 * @Description: 强制转换可由编译器隐式地推导并执行，也可以通过声明不同的生命周期的形式实现 https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/lifetime_coercion.html
 */

// 在这里，Rust 推导了一个尽可能短的生命周期
// 然后这两个引用都被强制转成这个生命周期
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长
// 在这里我们我们接受了一个 `&'a i32` 类型并返回一个 `&'b i32` 类型，这是强制转换得到的结果
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // 较长的生命周期

    {
        let second = 3; // 较短的生命周期

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}
