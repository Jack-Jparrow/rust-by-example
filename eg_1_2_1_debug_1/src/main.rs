/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-11 20:50:11
 * @LastEditTime: 2022-08-11 21:01:18
 * @LastEditors: Jack Jparrow
 * @Description: 所有 std 库类型都天生可以使用 {:?} 来打印 https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_debug.html
 */

// 推导 `Structure` 的 `fmt::Debug` 实现
// `Structure` 是一个包含单个 `i32` 的结构体
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // 使用 `{:?}` 打印和使用 `{}` 类似
    println!("{:?} months in a year", 12);
    println!(
        "{1:?}{0:?} is the {actor:?} name",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` 也可以打印
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式
    // 假如我只想展示一个 `7` 怎么办
    println!("Now {:?} will print!", Deep(Structure(7)));
}
