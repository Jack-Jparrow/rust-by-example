/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-24 20:23:38
 * @LastEditTime: 2022-08-24 20:32:27
 * @LastEditors: Jack Jparrow
 * @Description: 结构体的字段也是一个可见性的层次 https://rustwiki.org/zh-CN/rust-by-example/mod/struct_visibility.html
 */

mod my {
    // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 一个公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

fn main() {
    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = my::OpenBox {
        contents: "public information",
    };

    // 并且它们的字段可以正常访问到
    println!("The open box contains: {}", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造。
    // 报错！`ClosedBox` 含有私有字段。
    //let closed_box = my::ClosedBox { contents: "classified information" };

    // 不过带有私有字段的结构体可以使用公有的构造器来创建
    let _closedbox = my::ClosedBox::new("classified information");

    // 并且一个结构体中的私有字段不能访问到。
    // 报错！`content` 字段是私有的。
    //println!("The closed box contains: {}", _closed_box.contents);
}
