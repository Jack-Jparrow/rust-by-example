/**
 * @Author: 白银
 * @Date: 2022-08-27 22:14:33
 * @LastEditTime: 2022-08-27 22:34:47
 * @LastEditors: 白银
 * @Description: 约束的工作机制会产生这样的效果：即使一个 trait 不包含任何功能，你仍然可以用它 作为约束 https://rustwiki.org/zh-CN/rust-by-example/generics/bounds/testcase_empty.html
 */

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 这些函数只对实现了相应的 trait 的类型有效
// 事实上这些 trait 内部是空的，但这没有关系
fn read<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // 由于约束，`red()` 不能作用于 blue_jay （蓝松鸟），反过来也一样
    println!("A cardinal is {}", read(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
}
