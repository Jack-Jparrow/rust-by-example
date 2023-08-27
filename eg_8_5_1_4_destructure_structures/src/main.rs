/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-22 21:45:14
 * @LastEditTime: 2022-08-22 21:50:53
 * @LastEditors: Jack Jparrow
 * @Description: 解构 struct https://rustwiki.org/zh-CN/rust-by-example/flow_control/match/destructuring/destructure_structures.html
 */

fn main() {
    struct Foo {x: (u32, u32), y: u32}

    // 解构结构体的成员
    let foo = Foo {x: (1, 2), y: 3};
    let Foo{x: (a, b), y} = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // 可以解构结构体并重命名变量，成员顺序并不重要
    let Foo {y: i, x: j} = foo;

    println!("i = {:?}, j = {:?}", i, j);

    // 也可以忽略某些变量
    let Foo { y, .. } = foo;

    println!("y = {}", y);

    // 这将得到一个错误：模式中没有提及 `x` 字段
    // let Foo { y } = foo;
}