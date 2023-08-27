use std::convert::From;

/**
 * @Author: Jack Jparrow
 * @Date: 2022-08-20 20:51:50
 * @LastEditTime: 2022-08-20 21:04:15
 * @LastEditors: Jack Jparrow
 * @Description: From 可以为我们自己的类型定义转换机制 https://rustwiki.org/zh-CN/rust-by-example/conversion/from_into.html
 */

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);

    println!("My number is {:?}", num);
}
