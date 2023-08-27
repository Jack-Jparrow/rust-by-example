use std::fmt::Debug;

/**
 * @Author: 白银
 * @Date: 2022-08-28 20:15:36
 * @LastEditTime: 2022-08-28 20:49:35
 * @LastEditors: 白银
 * @Description: 约束也可以使用 where 分句来表达，它放在 { 的前面，而不需写在类型第一次出现 之前 https://rustwiki.org/zh-CN/rust-by-example/generics/where.html
 */

trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），或者改用另一种间接的方法
impl<T> PrintInOption for T
where
    Option<T>: Debug
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
