/**
 * @Author: 白银
 * @Date: 2022-08-27 22:00:08
 * @LastEditTime: 2022-08-27 22:02:52
 * @LastEditors: 白银
 * @Description: trait 也可以是泛型的 https://rustwiki.org/zh-CN/rust-by-example/generics/gen_trait.html
 */

// 不可复制的类型
struct Empty;
struct Null;

// `T` 的泛型 trait
trait DoubleDrop<T> {
    // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事
    fn double_drop(self, _: T);
}

// 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>`
impl<T, U> DoubleDrop<T> for U {
    // 此方法获得两个传入参数的所有权，并释放它们
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // 释放 `empty` 和 `null`
    empty.double_drop(null);

    //empty;
    //null;
}
