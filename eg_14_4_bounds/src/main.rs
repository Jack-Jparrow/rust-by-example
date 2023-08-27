use std::fmt::Debug;

/**
 * @Author: 白银
 * @Date: 2022-08-27 22:03:29
 * @LastEditTime: 2022-08-27 22:12:29
 * @LastEditors: 白银
 * @Description: 约束的另一个作用是泛型的实例可以访问作为约束的 trait 的方法 https://rustwiki.org/zh-CN/rust-by-example/generics/bounds.html
 */

// 这个 trait 用来实现打印标记：`{:?}`
trait HashArea {
    fn area(&self) -> f64;
}

impl HashArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// 泛型 `T` 必须实现 `Debug` 。只要满足这点，无论什么类型都可以让下面函数正常工作
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` 必须实现 `HasArea`。任意符合该约束的泛型的实例都可访问 `HasArea` 的 `area` 函数
fn area<T: HashArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ 试一试：取消上述语句的注释。
    // | 报错：未实现 `Debug` 或 `HasArea`
}
