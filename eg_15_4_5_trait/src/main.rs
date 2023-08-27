/**
 * @Author: 白银
 * @Date: 2022-08-30 21:12:46
 * @LastEditTime: 2022-08-30 21:20:34
 * @LastEditors: 白银
 * @Description: trait 方法中生命期的标注基本上与函数类似 https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime/trait.html
 */

// 带有生命周期标注的结构体
#[derive(Debug)]
#[allow(dead_code)]
struct Borrowed<'a> {
    x: &'a i32,
}

// 给 impl 标注生命周期
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn main() {
    let b: Borrowed = Default::default();

    println!("b is {:?}", b);
}
