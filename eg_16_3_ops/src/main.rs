use std::ops;

/**
 * @Author: 白银
 * @Date: 2022-08-31 19:34:29
 * @LastEditTime: 2022-08-31 20:02:05
 * @LastEditors: 白银
 * @Description: 在 Rust 中，很多运算符可以通过 trait 来重载 https://rustwiki.org/zh-CN/rust-by-example/trait/ops.html
 */

struct Foo;
struct Bar;
#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;

// `std::ops::Add` trait 用来指明 `+` 的功能，这里我们实现 `Add<Bar>`，它是用于把对象和 `Bar` 类型的右操作数（RHS）加起来的 `trait`
// 下面的代码块实现了 `Foo + Bar = FooBar` 这样的运算
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// 通过颠倒类型，我们实现了不服从交换律的加法
// 这里我们实现 `Add<Foo>`，它是用于把对象和 `Foo` 类型的右操作数加起来的 trait
// 下面的代码块实现了 `Bar + Foo = BarFoo` 这样的运算
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> Self::Output {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
