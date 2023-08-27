/**
 * @Author: 白银
 * @Date: 2022-08-29 21:05:41
 * @LastEditTime: 2022-08-29 21:30:52
 * @LastEditors: 白银
 * @Description: 生命周期（lifetime）是这样一种概念，编译器（中的借用检查器）用它来保证所有的借用都是有效的 https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime.html#%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F
 */

// 下面使用连线来标注各个变量的创建和销毁，从而显示出生命周期
// `i` 的生命周期最长，因为它的作用域完全覆盖了 `borrow1` 和 `borrow2` 的。`borrow1` 和 `borrow2` 的周期没有关联，因为它们各不相交
fn main() {
    let i = 3; // `i` 的生命周期开始──────────────────────────────────────

    {
        let borrow1 = &i; // `borrow1` 的生命周期开始..........

        println!("borrow1: {}", borrow1);
    } // `borrow1` 的生命周期结束....................................

    {
        let borrow2 = &i; // `borrow2` 的生命周期开始**********

        println!("borrow2: {}", borrow2);
    } // `borrow2` 的生命周期结束************************************
} // `i` 的生命周期结束────────────────────────────────────────────────────────
